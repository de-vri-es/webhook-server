use ::actix::prelude::*;
use ::actix_web::*;

use ::log::info;
use ::std::collections::HashMap;

use crate::messages::NewTask;
use crate::queue_actor::QueueActor;
use crate::template::verify_template_parameters;

/// State of the actix-web application
struct AppState {
    queue_actor: Addr<QueueActor>,
}

/// Index route
fn webhook(
    data: web::Data<AppState>,
    query: web::Query<HashMap<String, String>>,
    path_info: web::Path<String>,
) -> Result<HttpResponse, HttpResponse> {
    // Verify that the parameters match the required parameters in the template string
    let params = query.into_inner();
    let webhook_name = path_info.into_inner();

    info!("");
    info!("Incoming webhook for \"{}\":", webhook_name);
    let command = verify_template_parameters("ls {{rofl}}".to_string(), &params)?;

    // Create a new task with the checked parameters and webhook name
    let new_task = NewTask {
        name: webhook_name,
        parameters: params,
        command: command,
    };

    // Send the task to the actor managing the queue
    data.queue_actor.do_send(new_task);

    Ok(HttpResponse::Ok().finish())
}

/// Initialize the web server
/// Move the address of the queue actor inside the AppState for further dispatch
/// of tasks to the actor
pub fn init_web_server(queue_actor: Addr<QueueActor>) {
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                queue_actor: queue_actor.clone(),
            })
            .service(web::resource("/webhook/{webhook_name}").to(webhook))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .start();
}
