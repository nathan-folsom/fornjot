use crate::{
    objects::{handles::HandleSet, Face, FaceSet, HandleIter, Region, Surface},
    operations::Insert,
    services::Services,
    storage::Handle,
};

/// A 2-dimensional shape
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Sketch {
    regions: HandleSet<Region>,
}

impl Sketch {
    /// Construct an empty instance of `Sketch`
    pub fn new(regions: impl IntoIterator<Item = Handle<Region>>) -> Self {
        Self {
            regions: regions.into_iter().collect(),
        }
    }

    /// Access the regions of the sketch
    pub fn regions(&self) -> HandleIter<Region> {
        self.regions.iter()
    }

    /// Apply the regions of the sketch to some [`Surface`]
    pub fn faces(
        &self,
        surface: Handle<Surface>,
        services: &mut Services,
    ) -> FaceSet {
        self.regions
            .iter()
            .map(|region| {
                Face::new(surface.clone(), region.clone()).insert(services)
            })
            .collect()
    }
}
