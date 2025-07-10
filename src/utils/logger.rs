use std::{io, fs::File, sync::{OnceLock, Mutex}};
use env_logger::Target;

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

        // 配置并初始化env_logger
        env_logger::Builder::new()
            .target(Target::Pipe(Box::new(multi_writer)))
            .init();

        log::info!("Logger initialized successfully");
    });
}