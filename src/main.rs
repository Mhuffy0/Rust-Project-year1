use std::path::PathBuf;
use open;
use std::fs;
use std::time::Instant;
use iced::widget::image;
use iced::widget::image::Handle;
use iced::widget::{
    button, center, checkbox,Column, Container, Row, column, horizontal_rule,
    horizontal_space, pick_list, progress_bar, row,
    scrollable, slider, text, text_input, toggler,
    vertical_rule, vertical_space, Text, Scrollable, Theme, Image
};
use iced::Alignment::Center;
use iced::{Element, Length, Fill, Color, Size, Subscription, Renderer, Application, Font};
use rfd::FileDialog;
use DiskScanner::scanner::{drive_usage, scan_directory, compute_folder_stats};
use std::default::Default;
use DiskScanner::scanner::FileEntry;

pub fn main() -> iced::Result {
    iced::application("Disk Scanner", Styling::update, Styling::view)
        .theme(Styling::theme)
        .run()
}

// Define the pages of your application
#[derive(Debug, Clone, PartialEq)]
enum Page {
    Home,
    Normal,
}

impl Default for Page {
    fn default() -> Self {
        Page::Home
    }
}

#[derive(Debug, Clone)]
enum Message {
    FilePressed,
    FolderSelected(Option<PathBuf>),
    SelectItem(usize),
    Back,
    OpenFile(PathBuf),
    DeleteFile(PathBuf),
    SelectFile(usize), // Index of selected item in the list
}

#[derive(Default, Clone)]
struct Styling {
    theme: Theme,
    selected_path: Option<PathBuf>,
    files_name: Vec<FileEntry>, 
    description: Vec<String>,
    total: Vec<String>,
    usage_percentage: f64, 
    path_name: String,
    current_page: Page, // Track the current page
    Root_path: Option<PathBuf>,
    files: Vec<FileEntry>,
    selected_index: Option<usize>,
    last_click_time: Option<std::time::Instant>,
}

impl Styling {
    fn update(&mut self, message: Message) {
        match message {
            Message::FilePressed => {
                let folder = FileDialog::new().pick_folder();
                self.update(Message::FolderSelected(folder));
            }
            Message::FolderSelected(Some(path)) => {
                self.selected_path = Some(path.clone());
                self.current_page = Page::Normal; // Move to Normal page directly
                
                // Update files in the directory
                self.files_name = scan_directory(&path); // Scan directory
                
                // Get drive usage stats
                let (descriptions, usage_percentage) = drive_usage(&path);
                self.description = descriptions;
                self.usage_percentage = usage_percentage; // Store the usage percentage for the progress bar
                
                self.path_name = path.to_string_lossy().to_string();
                self.load_folder_contents(&path); // Load folder contents
                self.Root_path = Some(path.clone());
            }

            Message::FolderSelected(None) => {
                println!("No folder selected.");
            }

            Message::OpenFile(path) => {
                // join path valid
                if let Some(selected_path) = &self.selected_path {
                    let full_path = selected_path.join(path);
            
                    println!("Attempting to open: {}", full_path.display());
            
                    // Attempt to open 
                    if let Err(e) = open::that(full_path) {
                        println!("Failed to open file/folder: {}", e);
                    }
                } else {
                    println!("Selected path is not set.");
                }
            },

            Message::SelectFile(index) => {
                self.selected_index = Some(index);  // Update selected index
            },

            Message::DeleteFile(path) => {

                if let Some(selected_path) = &self.selected_path {
                    let full_path = selected_path.join(path);
            
                    println!("Attempting to delete: {}", full_path.display());
            
                    // Attempt to delete the file
                    if let Err(e) = std::fs::remove_file(&full_path) {
                        println!("Failed to delete file: {}", e);
                    } else {
                        //remove from the list
                        self.files.retain(|file| file.path != full_path);
                    }
                } else {
                    println!("Selected path is not set.");
                }
            },
            Message::SelectItem(index) => {
                if let Some(selected_item) = self.files_name.get(index) {
                    if let Some(last_click) = self.last_click_time {
                        // Double-click
                        if last_click.elapsed().as_secs_f32() < 0.3 {
                            //detected
                            if selected_item.is_folder {
                                let new_path = self.selected_path.as_ref().map(|path| path.join(&selected_item.name));
                                if let Some(path) = new_path {
                                    self.load_folder_contents(&path);
                                    self.path_name = path.to_string_lossy().to_string();
                                }
                            } else {
                                self.description = vec![
                                    format!("Name : {} ", selected_item.name),
                                    format!("Size: {} Mb", selected_item.size),
                                    format!("Last Modified: {}", selected_item.modified),
                                ];
                            }
                        } else {
                            // Single-click
                            self.selected_index = Some(index);
                        }
                    }

                    // Update the last click time to the current time
                    self.last_click_time = Some(Instant::now());
                }
            }

            Message::Back => {
                if let Some(current_path) = &self.selected_path {
                    if let Some(root_path) = &self.Root_path {
                        if current_path != root_path {
                            
                            if let Some(parent) = current_path.parent() {
                                
                                self.selected_path = Some(parent.to_path_buf());
            
                                let path_to_load = self.selected_path.clone().unwrap(); // Safe to unwrap here
            
                                self.load_folder_contents(&path_to_load);
                            }
                        }
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self.current_page {
            Page::Home => self.view_home(),     
            Page::Normal => self.view_normal(),
        }
    }

    // Home page view
    fn view_home(&self) -> Element<Message> {

        //GOOFY IMAGE
        let image_path = std::path::Path::new("assets/image.jpg");
        let img_handle = image::Handle::from_path(image_path);

        let file_button = button("Select Folder")
            .padding(20)
            .on_press(Message::FilePressed)
            .padding(20)
            .style(|theme: &Theme, status| {
        
                match status {
                    button::Status::Hovered => {
                        button::Style::default()
                           .with_background(Color::from_rgb(0.1, 0.4, 0.1))
                    }

                    button::Status::Active => {
                        button::Style::default()
                           .with_background(Color::from_rgb(0.2, 0.6, 0.2))
                    }

                    button::Status::Pressed => {
                        button::Style::default()
                           .with_background(Color::from_rgb(0.4, 0.8, 0.4))
                    }

                    _ => button::primary(theme, status),
                }
            });

        row![
            column![
            image(img_handle)
                .width(300)
                .height(300),
            horizontal_rule(20),
            file_button,
            ]
            .width(Length::Fill)
            .align_x(Center)
        ]
        .align_y(iced::alignment::Vertical::Center)
        .height(Fill)
        .spacing(20)
        .padding(20)
        .into()
    }

    // Normal page view
    fn view_normal(&self) -> Element<Message> {

        let usage_bar = progress_bar(0.0..=100.0, self.usage_percentage as f32)
            .width(300)
            .height(20);


        let file_button = button("Select Folder")
            .padding(20)
            .on_press(Message::FilePressed)
            .padding(20)
            .style(|theme: &Theme, status| {
                match status {
                    button::Status::Active => {
                        button::Style::default()
                        .with_background(Color::from_rgb(0.5, 0.5, 0.5))
                    }
                    button::Status::Hovered => {
                        button::Style::default()
                        .with_background(Color::WHITE)
                    }
                    button::Status::Pressed => {
                        button::Style::default()
                        .with_background(Color::from_rgb(0.4, 0.8, 0.4))
                    }
                    _ => button::primary(theme, status),
                }
            });

        let back_button = if let Some(current_path) = &self.selected_path {
            if let Some(root_path) = &self.Root_path {
                if current_path == root_path {
                    None
                } else {
                    Some(button("Go Back").on_press(Message::Back)
                    .padding(20)
                    .style(|theme: &Theme, status| {
                        match status {
                            button::Status::Active => {
                                button::Style::default()
                                .with_background(Color::from_rgb(0.5, 0.5, 0.5))
                            }
                            button::Status::Hovered => {
                                button::Style::default()
                                .with_background(Color::WHITE)
                            }
                            button::Status::Pressed => {
                                button::Style::default()
                                .with_background(Color::from_rgb(0.4, 0.8, 0.4))
                            }
                            _ => button::primary(theme, status),
                        }
                    }))
                }
            } else {
                None
            }
        } else {
            None
        };

        let total_usage = scrollable(
            column(self.total.iter().map(|desc| text(desc).size(16).into())).spacing(10),
        )
        .height(Length::Shrink);

        

        let list = scrollable(
            column(self.files_name.iter().enumerate().map(|(index, file_entry)| {
                let is_selected = self.selected_index == Some(index);
                let folder_path = std::path::Path::new("assets/folder.png");
                let file_path = std::path::Path::new("assets/text.png");

                let folder_handle = image::Handle::from_path(folder_path);
                let file_handle = image::Handle::from_path(file_path);

                //icon handler
                let icon_handle = if file_entry.is_folder {
                    folder_handle 
                } else {
                    file_handle
                };

                row![
                    button(
                        row![
                        image(icon_handle)
                        .width(30)
                        .height(30),
                        horizontal_space().width(10),
                        text(&file_entry.name)
                ])
                        .on_press(Message::SelectItem(index))
                        .padding(15)
                        .style(|theme: &Theme, status| {
                            match status {
                                button::Status::Active => {
                                    button::Style::default()
                                    .with_background(Color::from_rgb(0.5, 0.5, 0.5))
                                }
                                button::Status::Hovered => {
                                    button::Style::default()
                                    .with_background(Color::WHITE)
                                }
                                button::Status::Pressed => {
                                    button::Style::default()
                                    .with_background(Color::from_rgb(0.4, 0.8, 0.4))
                                }
                                _ => button::primary(theme, status),
                            }
                        }),

                        vertical_space()
                        .height(10),

                    // Only show file details if it's not a folder
                    if !file_entry.is_folder {
                        Some(text(format!("{} ", file_entry.size)))
                    } else {
                        None
                    }.map_or(text(""), |text_element| text_element),

                    if is_selected {
                        row![
                            button(text("Open"))
                                .on_press(Message::OpenFile(PathBuf::from(file_entry.name.clone())))
                                .padding(10)
                                .style(|theme: &Theme, status| {
                                    match status {
                                        button::Status::Active => {
                                            button::Style::default()
                                            .with_background(Color::from_rgb(0.5, 0.5, 0.5))
                                        }
                                        button::Status::Hovered => {
                                            button::Style::default()
                                            .with_background(Color::WHITE)
                                        }
                                        button::Status::Pressed => {
                                            button::Style::default()
                                            .with_background(Color::from_rgb(0.4, 0.8, 0.4))
                                        }
                                        _ => button::primary(theme, status),
                                    }
                                }),

                            button(text("Delete"))
                                .on_press(Message::DeleteFile(PathBuf::from(file_entry.name.clone())))
                                .padding(10)
                                .style(|theme: &Theme, status| {
                                    match status {
                                        button::Status::Active => {
                                            button::Style::default()
                                            .with_background(Color::from_rgb(0.5, 0.5, 0.5))
                                        }
                                        button::Status::Hovered => {
                                            button::Style::default()
                                            .with_background(Color::WHITE)
                                        }
                                        button::Status::Pressed => {
                                            button::Style::default()
                                            .with_background(Color::from_rgb(0.4, 0.8, 0.4))
                                        }
                                        _ => button::primary(theme, status),
                                    }
                                }),
                                        ]
                                        .spacing(10)
                                    } else {
                                        row![] // Show an empty row if not selected
                                    }
                                ]
                                .spacing(10)
                                .into()

                            }))
                            .spacing(0)
                        )
                        
                        .height(Length::Fill);  


        let desc = scrollable(
            column(self.description.iter().map(|desc| text(desc).size(16).into())).spacing(10),
        )
        .height(Length::Fill);

        column![
            row![
                file_button,
                horizontal_space().width(10),
                back_button.map_or(iced::widget::text("").into(), |button| Element::from(button)),
            ],
            row![
                text("Total Size")
                    .size(30)
                    .align_x(iced::alignment::Horizontal::Left),
                horizontal_space().width(10),
                text("Description")
                    .size(30)
                    .align_x(iced::alignment::Horizontal::Center)
                    .width(Length::Fill),
                text("Items")
                    .size(30)
                    .align_x(iced::alignment::Horizontal::Left)
                    .width(Length::Fill),
            ]
            .padding(10),

            row![
                text("File Path : ")
                .size(20),
                text(&self.path_name)
                    .size(20)
                    .align_x(iced::alignment::Horizontal::Left),
                horizontal_space().width(30),
            ]
            .padding(10),
            row![
                column![
                    total_usage,
                    vertical_space().height(20),
                    usage_bar,
                ],
                horizontal_space().width(50),
                vertical_rule(30),
                desc,
                horizontal_space().width(30),
                vertical_rule(30),
                list,
            ]
            .padding(10),
        ]
        .spacing(5)
        .padding(10)
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn load_folder_contents(&mut self, path: &PathBuf) {
        self.selected_path = Some(path.clone());
        
        // Avoid scanning directories repeatedly
        if let Some(stats) = compute_folder_stats(path) {
            // Update description with detailed stats
            self.description = vec![
                format!("Name: {}", stats.file_name),
                format!("Total size: {}", stats.total_size),
                format!("Allocated size: {}", stats.allocated_size),
                format!("Item count: {}", stats.item_count),
                format!("File count: {}", stats.file_count),
                format!("Subdirectory count: {}", stats.subdir_count),
                format!("Last modified: {}", stats.last_modified),
            ];
    
            // Update total stats for the view
            let (drive_usage_info, usage_percentage) = drive_usage(path);
            self.total = drive_usage_info;

        } else {
            self.description = vec!["Failed to compute folder stats".to_string()];
            self.total = vec!["Failed to compute total stats".to_string()];
        }
    
        // Cache the file list only when the path changes
        self.files_name = scan_directory(path);
        self.path_name = path.to_string_lossy().to_string();
    }
    
}