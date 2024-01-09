use stellwerksim::protocol::EventType;
use stellwerksim::Plugin;

#[tokio::main()]
async fn main() {
    let plugin = Plugin::builder()
        .name("Example Plugin")
        .version("1.0.0")
        .author("uwumarie")
        .connect()
        .await
        .unwrap();
    let time = plugin.simulator_time().await.unwrap();
    println!("{time:?}");

    let info = plugin.system_info().await.unwrap();
    println!("{info:#?}");

    let platform_list = plugin.platform_list().await.unwrap();
    println!("{platform_list:#?}");

    let train_list = plugin.train_list().await.unwrap();
    println!("{train_list:#?}");

    let train_details = plugin
        .train_details(&train_list.first().unwrap().id)
        .await
        .unwrap();
    println!("{train_details:#?}");

    let train_timetable = plugin
        .train_timetable(&train_list.first().unwrap().id)
        .await
        .unwrap();
    println!("{train_timetable:#?}");

    let ways = plugin.ways().await.unwrap();
    println!("{ways:#?}");

    let mut receivers = Vec::new();
    for train in train_list {
        let receiver = plugin
            .subscribe_events(&train.id, EventType::all())
            .await
            .unwrap();

        receivers.push(receiver);
    }

    let handles = receivers.into_iter().map(|mut receiver| {
        tokio::spawn(async move {
            println!("received event: {:#?}", receiver.recv().await);
        })
    });

    let _ = futures::future::select_all(handles).await;
}
