use tracing::info;
use tracing_subscriber;
use tracing_appender;

fn main() {
    let file_appender = tracing_appender::rolling::hourly("./logs", "hello-tracing.log");
    let (non_blocking, _) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();
    
    let number_of_yaks = 3;
    info!(number_of_yaks, "preparing to shave yaks");
}
