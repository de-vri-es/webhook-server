use ::actix::prelude::*;
use ::log::info;

use crate::messages::*;
use crate::task_actor::TaskActor;


pub struct QueueActor {
    pub task_actor: Addr<TaskActor>,
    pub own_addr: Option<Addr<Self>>
}


impl Actor for QueueActor {
    type Context = Context<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        self.own_addr = Some(context.address());
        info!("Queue management actor started up");
    }
}

impl Handler<NewTask> for QueueActor {
    type Result = ();

    fn handle(&mut self, new_task: NewTask, context: &mut Self::Context) {
        info!("Got new Task: {}", new_task.id);

        self.dispatch_task(new_task);
    }
}


impl QueueActor {
    fn dispatch_task(&mut self, new_task: NewTask) {
        let addr = self.own_addr.as_ref().unwrap().clone();

        let start_task = StartTask {
            command: new_task.command,
            cwd: "/".to_string(),
            queue_actor: addr,
        };

        self.task_actor.do_send(start_task);
    }
}
