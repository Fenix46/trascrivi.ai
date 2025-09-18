use anyhow::Result;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use crate::models::{Transcription, ExportFormat, ExportType};
use crate::storage::StorageService;

pub struct ExportService {
    storage: StorageService,
}

impl ExportService {
    pub fn new(storage: StorageService) -> Self {
        Self { storage }
    }

    pub async fn export_transcription(
        &self,
        transcription: &Transcription,
        format: &ExportFormat,
    ) -> Result<String> {
        match format.format_type {
            ExportType::Pdf => self.export_to_pdf(transcription, format).await,
            ExportType::Docx => self.export_to_docx(transcription, format).await,
            ExportType::Txt => self.export_to_txt(transcription, format).await,
            ExportType::Markdown => self.export_to_markdown(transcription, format).await,
        }
    }

    async fn export_to_pdf(
        &self,
        transcription: &Transcription,
        format: &ExportFormat,
    ) -> Result<String> {
        let filename = format!("{}_transcription.pdf", transcription.id);
        let file_path = self.storage.get_export_path(&filename);

        let (doc, page1, layer1) = PdfDocument::new("Transcription", Mm(210.0), Mm(297.0), "Layer 1");
        let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
        let regular_font = doc.add_builtin_font(BuiltinFont::Helvetica)?;

        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Title
        current_layer.use_text(&transcription.title, 24.0, Mm(20.0), Mm(260.0), &font);

        // Date and duration
        let date_str = transcription.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
        let duration_str = format!("Duration: {}s", transcription.duration);
        current_layer.use_text(&date_str, 12.0, Mm(20.0), Mm(240.0), &regular_font);
        current_layer.use_text(&duration_str, 12.0, Mm(20.0), Mm(230.0), &regular_font);

        let mut y_position = 200.0;

        // Chapters
        if format.include_chapters && !transcription.chapters.is_empty() {
            for chapter in &transcription.chapters {
                if y_position < 40.0 {
                    // Create new page if running out of space
                    let (page_id, layer_id) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                    let _new_layer = doc.get_page(page_id).get_layer(layer_id);
                    y_position = 260.0;
                }

                // Chapter title
                current_layer.use_text(&chapter.title, 16.0, Mm(20.0), Mm(y_position), &font);
                y_position -= 10.0;

                if format.include_timestamps {
                    let timestamp = format!("Start: {:.1}s", chapter.start_time);
                    current_layer.use_text(&timestamp, 10.0, Mm(20.0), Mm(y_position), &regular_font);
                    y_position -= 10.0;
                }

                // Chapter content
                let lines = Self::wrap_text(&chapter.content, 80);
                for line in lines {
                    if y_position < 40.0 {
                        let (page_id, layer_id) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                        let _new_layer = doc.get_page(page_id).get_layer(layer_id);
                        y_position = 260.0;
                    }
                    current_layer.use_text(&line, 11.0, Mm(20.0), Mm(y_position), &regular_font);
                    y_position -= 8.0;
                }
                y_position -= 10.0; // Extra space between chapters
            }
        } else {
            // Full text without chapters
            let lines = Self::wrap_text(&transcription.raw_text, 80);
            for line in lines {
                if y_position < 40.0 {
                    let (page_id, layer_id) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                    let _new_layer = doc.get_page(page_id).get_layer(layer_id);
                    y_position = 260.0;
                }
                current_layer.use_text(&line, 11.0, Mm(20.0), Mm(y_position), &regular_font);
                y_position -= 8.0;
            }
        }

        doc.save(&mut BufWriter::new(File::create(&file_path)?))?;

        Ok(file_path.to_string_lossy().to_string())
    }

    async fn export_to_docx(
        &self,
        transcription: &Transcription,
        format: &ExportFormat,
    ) -> Result<String> {
        let filename = format!("{}_transcription.docx", transcription.id);
        let file_path = self.storage.get_export_path(&filename);

        // For now, create a simple text file with .docx extension
        // In a real implementation, you would use docx-rs crate
        let mut content = String::new();

        content.push_str(&format!("# {}\n\n", transcription.title));
        content.push_str(&format!("Date: {}\n", transcription.created_at.format("%Y-%m-%d %H:%M:%S")));
        content.push_str(&format!("Duration: {}s\n\n", transcription.duration));

        if format.include_chapters && !transcription.chapters.is_empty() {
            for chapter in &transcription.chapters {
                content.push_str(&format!("## {}\n", chapter.title));
                if format.include_timestamps {
                    content.push_str(&format!("Start: {:.1}s\n", chapter.start_time));
                }
                content.push_str(&format!("{}\n\n", chapter.content));
            }
        } else {
            content.push_str(&transcription.raw_text);
        }

        tokio::fs::write(&file_path, content).await?;

        Ok(file_path.to_string_lossy().to_string())
    }

    async fn export_to_txt(
        &self,
        transcription: &Transcription,
        format: &ExportFormat,
    ) -> Result<String> {
        let filename = format!("{}_transcription.txt", transcription.id);
        let file_path = self.storage.get_export_path(&filename);

        let mut content = String::new();

        content.push_str(&format!("{}\n", transcription.title));
        content.push_str(&format!("Date: {}\n", transcription.created_at.format("%Y-%m-%d %H:%M:%S")));
        content.push_str(&format!("Duration: {}s\n\n", transcription.duration));

        if format.include_chapters && !transcription.chapters.is_empty() {
            for chapter in &transcription.chapters {
                content.push_str(&format!("--- {} ---\n", chapter.title));
                if format.include_timestamps {
                    content.push_str(&format!("Start: {:.1}s\n", chapter.start_time));
                }
                content.push_str(&format!("{}\n\n", chapter.content));
            }
        } else {
            content.push_str(&transcription.raw_text);
        }

        tokio::fs::write(&file_path, content).await?;

        Ok(file_path.to_string_lossy().to_string())
    }

    async fn export_to_markdown(
        &self,
        transcription: &Transcription,
        format: &ExportFormat,
    ) -> Result<String> {
        let filename = format!("{}_transcription.md", transcription.id);
        let file_path = self.storage.get_export_path(&filename);

        let mut content = String::new();

        content.push_str(&format!("# {}\n\n", transcription.title));
        content.push_str(&format!("**Date:** {}\n", transcription.created_at.format("%Y-%m-%d %H:%M:%S")));
        content.push_str(&format!("**Duration:** {}s\n\n", transcription.duration));

        if format.include_chapters && !transcription.chapters.is_empty() {
            for chapter in &transcription.chapters {
                content.push_str(&format!("## {}\n", chapter.title));
                if format.include_timestamps {
                    content.push_str(&format!("*Start: {:.1}s*\n\n", chapter.start_time));
                }
                content.push_str(&format!("{}\n\n", chapter.content));
            }
        } else {
            content.push_str(&transcription.raw_text);
        }

        tokio::fs::write(&file_path, content).await?;

        Ok(file_path.to_string_lossy().to_string())
    }

    fn wrap_text(text: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            if current_line.len() + word.len() + 1 > width {
                if !current_line.is_empty() {
                    lines.push(current_line.clone());
                    current_line.clear();
                }
            }

            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }
}