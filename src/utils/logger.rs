use std::{env, error::Error, fs::File, io, sync::{Mutex, OnceLock}};
use env_logger::Target;
use log::LevelFilter;
use serde::{Serialize, Deserialize};
// 1. 定义日志写入器（支持文件+控制台双输出）
struct MultiWriter {
    file: Mutex<File>,
    stdout: io::Stdout,
}

impl io::Write for MultiWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // 线程安全写入（加锁）
        let mut file = self.file.lock().unwrap();
        file.write_all(buf)?;
        self.stdout.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut file = self.file.lock().unwrap();
        file.flush()?;
        self.stdout.flush()?;
        Ok(())
    }
}

// 2. 全局日志初始化器（静态单例）
static LOGGER_INIT: OnceLock<()> = OnceLock::new();

// 3. 安全的全局初始化函数
pub fn init_global_logger_once(output_file: &'static str) {
    LOGGER_INIT.get_or_init(|| {
        let log_file = File::create(output_file)
            .expect("Failed to create log file");
        
        let multi_writer = MultiWriter {
            file: Mutex::new(log_file),
            stdout: io::stdout(),
        };

        let level = if let Ok(level) = env::var("RUST_LOG") {
            match level.to_lowercase().as_str() {
                "error" => LevelFilter::Error,
                "warn" | "warning" => LevelFilter::Warn,
                "info" => LevelFilter::Info,
                "debug" => LevelFilter::Debug,
                "trace" => LevelFilter::Trace,
                _ => LevelFilter::Info, // 默认级别
            }
        } else {
            LevelFilter::Info
        };// 默认级别

        // 配置并初始化env_logger
        env_logger::Builder::new()
            .target(Target::Pipe(Box::new(multi_writer)))
            .filter_level(level)
            .init();

        log::info!("Logger initialized successfully");
    });
}

pub trait TraceLog: Serialize +  for<'de> Deserialize<'de> {
    fn store_trace_file(self, filename: &'static str) -> Result<(), Box<dyn Error>>;
    fn get_trace(filename: &'static str) -> Result<Self, Box<dyn Error>>;
}
