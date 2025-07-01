use std::fs;
use std::io;

pub struct FileContent {
    content: String,
    lines: Vec<String>,

    current_line_index: usize,
}

impl FileContent {
    
    pub fn from_file(path: &str) -> Result<Self, io::Error> {

        let content = fs::read_to_string(path)?;
        let lines = content.lines().map(|s| s.to_string()).collect();
                
        Ok(FileContent { content, lines, current_line_index: 0 })
    }

    pub fn count_lines(&self) -> usize {

        self.lines.len()
    }

    fn find_line_containing(&self, pattern: &str) -> Option<&String> {

        self.lines.iter().find(|line| line.contains(pattern))
    }
    
    // Methods for traversing and parsing
    fn get_content(&self) -> &str {

        &self.content
    }

    pub fn get_current_line_index(&self) -> usize {

        if self.current_line_index > 0 {

            return self.current_line_index - 1; // Adjusting to return the last accessed line
        } else {

            0 // If no lines have been accessed, return 0
        }        
    }

    // index originates at 0
    pub fn get_line_by_index(&self, index: usize) -> Option<&String> {

        if index < self.lines.len() {
           
           return self.lines.get(index);
        } 
        
        None
    }
    
    fn get_lines(&self) -> &[String] {

        &self.lines
    }

    pub fn go_to_next_line(&mut self) -> Option<&String> {

        if self.current_line_index < self.lines.len() {
            
            let ret = Some(&self.lines[self.current_line_index]);    
            self.current_line_index += 1;       

            ret
        } else {

            self.current_line_index = 0; // Reset to the first line if at the end
            None // No next line available
        }
    }

    // index originates at 0
    pub fn set_current_line_index(&mut self, index: usize) {

        if index < self.lines.len() {
            self.current_line_index = index;
        } else {
            self.current_line_index = 0; // Reset to the first line if index is out of bounds
        }
    }
        
}