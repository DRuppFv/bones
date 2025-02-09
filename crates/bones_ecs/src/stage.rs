//! Implementation of stage abstraction for running collections of systems over a [`World`].

use crate::prelude::*;

/// An ordered collection of [`SystemStage`]s.
pub struct SystemStages {
    /// The stages in the collection, in the order that they will be run.
    pub stages: Vec<Box<dyn SystemStage>>,
}

impl SystemStages {
    /// Initialize the systems in the stages agains the [`World`].
    ///
    /// This must be called once before calling [`run()`][Self::run].
    pub fn initialize_systems(&mut self, world: &mut World) {
        for stage in &mut self.stages {
            stage.initialize(world);
        }
    }

    /// Execute the systems on the given `world`.
    ///
    /// > **Note:** You must call [`initialize_systems()`][Self::initialize_systems] once before
    /// > calling `run()` one or more times.
    pub fn run(&mut self, world: &World) -> SystemResult {
        for stage in &mut self.stages {
            stage.run(world)?;
        }

        Ok(())
    }

    /// Create a [`SystemStages`] collection, initialized with a stage for each [`CoreStage`].
    pub fn with_core_stages() -> Self {
        Self {
            stages: vec![
                Box::new(SimpleSystemStage::new(CoreStage::First)),
                Box::new(SimpleSystemStage::new(CoreStage::PreUpdate)),
                Box::new(SimpleSystemStage::new(CoreStage::Update)),
                Box::new(SimpleSystemStage::new(CoreStage::PostUpdate)),
                Box::new(SimpleSystemStage::new(CoreStage::Last)),
            ],
        }
    }

    /// Add a [`System`] to the stage with the given label.
    pub fn add_system_to_stage<Args, S: IntoSystem<Args>, L: StageLabel>(
        &mut self,
        label: L,
        system: S,
    ) -> &mut Self {
        let name = label.name();
        let id = label.id();
        let mut stage = None;

        for st in &mut self.stages {
            if st.id() == id {
                stage = Some(st);
            }
        }

        let Some(stage) = stage else {
            panic!("Stage with label `{}` ( {} ) doesn't exist.", name, id);
        };

        stage.add_system(system.system());

        self
    }
}

/// Trait for system stages. A stage is a
pub trait SystemStage: Sync + Send {
    /// The unique identifier for the stage.
    fn id(&self) -> Ulid;
    /// The human-readable name for the stage, used for error messages when something goes wrong.
    fn name(&self) -> String;
    /// Execute the systems on the given `world`.
    ///
    /// > **Note:** You must call [`initialize()`][Self::initialize] once before calling `run()` one
    /// > or more times.
    fn run(&mut self, world: &World) -> SystemResult;
    /// Initialize the contained systems for the given `world`.
    ///
    /// Must be called once before calling [`run()`][Self::run].
    fn initialize(&mut self, world: &mut World);

    /// Add a system to this stage.
    fn add_system(&mut self, system: System);
}

/// A collection of systems that will be run in order.
pub struct SimpleSystemStage {
    /// The unique identifier for the stage.
    pub id: Ulid,
    /// The human-readable name for the stage, used for error messages when something goes wrong.
    pub name: String,
    /// The list of systems in the stage.
    ///
    /// Each system will be run in the order that they are in in this list.
    pub systems: Vec<System>,
}

impl SimpleSystemStage {
    /// Create a new, empty stage, for the given label.
    pub fn new<L: StageLabel>(label: L) -> Self {
        Self {
            id: label.id(),
            name: label.name(),
            systems: Default::default(),
        }
    }
}

impl SystemStage for SimpleSystemStage {
    fn id(&self) -> Ulid {
        self.id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn run(&mut self, world: &World) -> SystemResult {
        for system in &mut self.systems {
            system.run(world)?;
        }

        Ok(())
    }

    fn initialize(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.initialize(world);
        }
    }

    fn add_system(&mut self, system: System) {
        self.systems.push(system);
    }
}

/// Trait for things that may be used to identify a system stage.
pub trait StageLabel {
    /// Returns the human-readable name of the label, used in error messages.
    fn name(&self) -> String;
    /// Returns a unique identifier for the stage.
    fn id(&self) -> Ulid;
}

/// A [`StageLabel`] for the 5 core stages.
#[derive(Copy, Clone, Debug)]
pub enum CoreStage {
    /// The first stage
    First,
    /// The second stage
    PreUpdate,
    /// The third stage
    Update,
    /// The fourth stage
    PostUpdate,
    /// The fifth stage
    Last,
}

impl StageLabel for CoreStage {
    fn name(&self) -> String {
        match self {
            CoreStage::First => "First",
            CoreStage::PreUpdate => "PreUpdate",
            CoreStage::Update => "Update",
            CoreStage::PostUpdate => "PostUpdate",
            CoreStage::Last => "Last",
        }
        .into()
    }

    fn id(&self) -> Ulid {
        match self {
            CoreStage::First => Ulid(2021715391084198804812356024998495966),
            CoreStage::PreUpdate => Ulid(2021715401330719559452824437611089988),
            CoreStage::Update => Ulid(2021715410160177201728645950400543948),
            CoreStage::PostUpdate => Ulid(2021715423103233646561968734173322317),
            CoreStage::Last => Ulid(2021715433398666914977687392909851554),
        }
    }
}
