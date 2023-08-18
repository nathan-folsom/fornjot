use std::ops::RangeInclusive;

use fj_math::Point;
use itertools::Itertools;

use crate::{
    geometry::{CurveBoundary, SurfacePath},
    objects::{Cycle, Edge},
    operations::{BuildEdge, Insert, UpdateCycle, UpdateEdge},
    services::Services,
    storage::Handle,
};

/// Join a [`Cycle`] to another
pub trait JoinCycle {
    /// Create a cycle that is joined to the provided edges
    #[must_use]
    fn add_joined_edges<Es>(&self, edges: Es, services: &mut Services) -> Self
    where
        Es: IntoIterator<
            Item = (Handle<Edge>, SurfacePath, CurveBoundary<Point<1>>),
        >,
        Es::IntoIter: Clone + ExactSizeIterator;

    /// Join the cycle to another
    ///
    /// Joins the cycle to the other at the provided ranges. The ranges specify
    /// the indices of the half-edges that are joined together.
    ///
    /// A modulo operation is applied to all indices before use, so in a cycle
    /// of 3 half-edges, indices `0` and `3` refer to the same half-edge. This
    /// allows for specifying a range that crosses the "seam" of the cycle.
    ///
    /// # Panics
    ///
    /// Panics, if the ranges have different lengths.
    ///
    /// # Assumptions
    ///
    /// This method makes some assumptions that need to be met, if the operation
    /// is to result in a valid shape:
    ///
    /// - **The joined half-edges must be coincident.**
    /// - **The locally defined curve coordinate systems of the edges must
    ///   match.**
    ///
    /// If either of those assumptions are not met, this will result in a
    /// validation error down the line.
    ///
    /// # Implementation Note
    ///
    /// The use of the `RangeInclusive` type might be a bit limiting, as other
    /// range types might be more convenient in a given use case. This
    /// implementation was chosen for now, as it wasn't clear whether the
    /// additional complexity of using `RangeBounds` would be worth it.
    ///
    /// A solution based on `SliceIndex` could theoretically be used, but that
    /// trait is partially unstable. In addition, it's not clear how it could be
    /// used generically, as it could yield a range (which can be iterated over)
    /// or a single item (which can not). This is not a hard problem in
    /// principle (a single item could just be an iterator of length 1), but I
    /// don't see it how to address this in Rust in a reasonable way.
    ///
    /// Maybe a custom trait that is implemented for `usize` and all range types
    /// would be the best solution.
    #[must_use]
    fn join_to(
        &self,
        other: &Cycle,
        range: RangeInclusive<usize>,
        other_range: RangeInclusive<usize>,
        services: &mut Services,
    ) -> Self;
}

impl JoinCycle for Cycle {
    fn add_joined_edges<Es>(&self, edges: Es, services: &mut Services) -> Self
    where
        Es: IntoIterator<
            Item = (Handle<Edge>, SurfacePath, CurveBoundary<Point<1>>),
        >,
        Es::IntoIter: Clone + ExactSizeIterator,
    {
        self.add_half_edges(edges.into_iter().circular_tuple_windows().map(
            |((prev, _, _), (half_edge, curve, boundary))| {
                Edge::unjoined(curve, boundary, services)
                    .replace_curve(half_edge.curve().clone())
                    .replace_start_vertex(prev.start_vertex().clone())
                    .insert(services)
            },
        ))
    }

    fn join_to(
        &self,
        other: &Cycle,
        range: RangeInclusive<usize>,
        range_other: RangeInclusive<usize>,
        services: &mut Services,
    ) -> Self {
        assert_eq!(
            range.end() - range.start(),
            range_other.end() - range_other.start(),
            "Ranges have different lengths",
        );

        let mut cycle = self.clone();

        for (index, index_other) in range.zip(range_other) {
            let index = index % self.len();
            let index_other = index_other % self.len();

            let half_edge = self
                .nth_edge(index)
                .expect("Index must be valid, due to use of `%` above");
            let half_edge_other = other
                .nth_edge(index_other)
                .expect("Index must be valid, due to use of `%` above");

            let vertex_a = other
                .half_edge_after(half_edge_other)
                .expect("Cycle must contain edge; just obtained edge from it")
                .start_vertex()
                .clone();
            let vertex_b = half_edge_other.start_vertex().clone();

            let next_edge = cycle
                .half_edge_after(half_edge)
                .expect("Cycle must contain edge; just obtained edge from it");

            let this_joined = half_edge
                .replace_curve(half_edge_other.curve().clone())
                .replace_start_vertex(vertex_a)
                .insert(services);
            let next_joined =
                next_edge.replace_start_vertex(vertex_b).insert(services);

            cycle = cycle
                .replace_half_edge(half_edge, this_joined)
                .replace_half_edge(next_edge, next_joined)
        }

        cycle
    }
}
