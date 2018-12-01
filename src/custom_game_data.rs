use amethyst::shred::{Dispatcher,DispatcherBuilder};
use amethyst::prelude::{DataInit,World};
use amethyst::core::{SystemBundle, ArcThreadPool};
use amethyst::Error;


pub enum DispatchData {
    Core,
    Menu,
    Gameplay,
}

pub struct CustomGameData<'a, 'b> {
    core_dispatcher: Dispatcher<'a, 'b>,
    running_dispatcher: Dispatcher<'a, 'b>,
    ui_dispatcher: Dispatcher<'a,'b>,
}

impl<'a, 'b> CustomGameData<'a, 'b> {
    pub fn update(&mut self, world: &World, running: bool) {
        if running {
            self.running_dispatcher.dispatch(&world.res);
        } else {
            self.ui_dispatcher.dispatch(&world.res);
        }
        self.core_dispatcher.dispatch(&world.res);
    }
}

pub struct CustomGameDataBuilder<'a, 'b> {
    pub core: DispatcherBuilder<'a, 'b>,
    pub running: DispatcherBuilder<'a, 'b>,
    pub ui: DispatcherBuilder<'a,'b>,
}

impl<'a, 'b> Default for CustomGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        CustomGameDataBuilder::new()
    }
}

impl<'a, 'b> CustomGameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        CustomGameDataBuilder {
            core: DispatcherBuilder::new(),
            running: DispatcherBuilder::new(),
            ui: DispatcherBuilder::new()
        }
    }

    pub fn with_bundle<B: SystemBundle<'a,'b>>(mut self, bundle: B,data: DispatchData) -> Result<Self, Error> {
        { 
            let mut disp = match data {
                DispatchData::Core => &mut self.core,
                DispatchData::Gameplay => &mut self.running,
                DispatchData::Menu => &mut self.ui,
            };
            bundle
                .build(disp)
                .map_err(|err| Error::Core(err))?;
        }
        Ok(self)
    }

}

impl<'a, 'b> DataInit<CustomGameData<'a, 'b>> for CustomGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> CustomGameData<'a, 'b> {
        // Get a handle to the `ThreadPool`.
        let pool = world.read_resource::<ArcThreadPool>().clone();

        let mut core_dispatcher = self.core.with_pool(pool.clone()).build();
        let mut running_dispatcher = self.running.with_pool(pool.clone()).build();
        let mut ui_dispatcher = self.ui.with_pool(pool.clone()).build();

        core_dispatcher.setup(&mut world.res);
        running_dispatcher.setup(&mut world.res);
        ui_dispatcher.setup(&mut world.res);

        CustomGameData { core_dispatcher, running_dispatcher, ui_dispatcher }
    }
}
