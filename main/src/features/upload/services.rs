use std::path::Path;

use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use futures::TryStreamExt;
use tokio::{fs::File, io::AsyncWriteExt};
pub struct UploadService;

impl UploadService {
    pub fn new() -> Self {
        Self
    }

    pub async fn upload(&self, mut payload: Multipart) -> Result<HttpResponse, Error> {
        // TODO: implement upload logic
        while let Some(mut field) = payload.try_next().await? {
            // 提前提取文件名，避免后续同时借用
            let filename = field
                .content_disposition()
                .and_then(|cd| cd.get_filename())
                .map(|name| name.to_string());

            if let Some(filename) = filename {
                // 定义文件保存的路径
                let filepath = format!("./tmp/{}", filename);
                let path = Path::new(&filepath);

                // 如果父目录不存在，则创建它
                if let Some(parent) = path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }

                // 创建目标文件，准备写入
                let mut file = File::create(path).await?;

                // 将上传的文件数据流式写入到目标文件中
                while let Some(chunk) = field.try_next().await? {
                    file.write_all(&chunk).await?;
                }

                return Ok(HttpResponse::Ok().body(format!("文件 {} 上传成功！", filename)));
            }
        }

        Ok(HttpResponse::BadRequest().body("未找到要上传的文件"))
    }
}
