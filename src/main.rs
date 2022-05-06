use std::io::Write;

#[tokio::main]
async fn main() {
    let mut time: u64 = tokio::fs::read_to_string("~/Library/tuimer/time.text")
        .await
        .unwrap()
        .parse()
        .unwrap();
    loop {
        let seconds = time % 60;
        let minutes = (time / 60) % 60;
        let hours = time / 3600;
        print!("{:02}:{:02}:{:02}\r", hours, minutes, seconds,);
        std::io::stdout().flush().unwrap();
        time += 1;
        tokio::spawn(async move {
            tokio::fs::write("~/Library/tuimer/time.text", time.to_string())
                .await
                .ok();
        });
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
