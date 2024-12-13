#[derive(Debug)]
// Define an enum `Segment` to represent either a file segment or a free space segment
enum Segment {
    File(usize, usize), 
    Free(usize),        
}

#[derive(Debug)]
// Define a struct `FileInfo` to store information about a file
struct FileInfo {
    file_id: usize, 
    start: usize,   
    size: usize,    
}

#[derive(Debug, Clone, Copy)]
// Define a struct `FreeSpan` to represent a span of free space
struct FreeSpan {
    start: usize,  
    length: usize, 
}

fn collect_files_and_free_spans(expanded: &Vec<Option<usize>>) -> (Vec<FileInfo>, Vec<FreeSpan>) {
    let mut files = Vec::new();        
    let mut free_spans = Vec::new();   
    let mut i = 0;                      
    let n = expanded.len();            

    // Iterate through the `expanded` vector
    while i < n {
        match expanded[i] {
            Some(file_id) => { // If the current block is part of a file
                let start = i; 
                let mut size = 1; 
                i += 1; 
                while i < n && expanded[i] == Some(file_id) {
                    size += 1;
                    i += 1;
                }
                files.push(FileInfo { file_id, start, size });
            }
            None => { // If the current block is free space
                let start = i; 
                let mut length = 1; 
                i += 1; 
                while i < n && expanded[i].is_none() {
                    length += 1;
                    i += 1;
                }
                free_spans.push(FreeSpan { start, length });
            }
        }
    }

    (files, free_spans) 
}

fn parse_input_in_segments(input: &str) -> Vec<Segment> {
    let mut segments = Vec::new(); 
    let mut chars = input.chars(); 
    let mut file_id = 0; 
    let mut is_file = true; 


    while let Some(c) = chars.next() {
        if !c.is_digit(10) {
            continue; 
        }
        let length = c.to_digit(10).unwrap() as usize; 
        if is_file {
            
            segments.push(Segment::File(file_id, length)); 
            file_id += 1; 
        } else {
            
            segments.push(Segment::Free(length)); 
        }
        is_file = !is_file; 
    }

    segments
}


fn expand_segments(segments: &Vec<Segment>) -> Vec<Option<usize>> {
    let mut expanded = Vec::new();
    for segment in segments {
        match segment {
            Segment::File(id, length) => {
                for _ in 0..*length {
                    expanded.push(Some(*id)); 
                }
            }
            Segment::Free(length) => {
                for _ in 0..*length {
                    expanded.push(None); 
                }
            }
        }
    }
    expanded 
}

fn calculate_checksum(expanded: &Vec<Option<usize>>) -> usize {
    let total = expanded.iter().enumerate().map(|(i, id)| match id {
        Some(id) => i * id, // Multiply the index by the file ID if occupied
        None => 0,          // Add zero if the block is free
    });
    total.sum() 
}

fn move_files(expanded: &mut Vec<Option<usize>>) {
    let mut start_index = 0; // Initialize the start pointer at the beginning
    let mut end_index = expanded.len() - 1; // Initialize the end pointer at the end

    // Continue until the start pointer meets the end pointer
    while start_index < end_index {
        if expanded[start_index].is_some() {
            start_index += 1; // Move the start pointer forward if the block is occupied
        } else if expanded[end_index].is_none() {
            end_index -= 1; // Move the end pointer backward if the block is free
        } else {
            // Swap the free block at `start_index` with the occupied block at `end_index`
            expanded.swap(start_index, end_index);
        }
    }
}

/// This function collects file and free span information, sorts the files,
/// and moves them to the leftmost suitable free spans.
/// It also updates the free spans accordingly.
fn move_files_part2(expanded: &mut Vec<Option<usize>>) {
    let (mut files, mut free_spans) = collect_files_and_free_spans(expanded); 
    
    // Sort files in descending order of `file_id` to prioritize moving larger or newer files first
    files.sort_unstable_by(|a, b| b.file_id.cmp(&a.file_id));

    for file in &files {
        // Find the position of the first free span that can accommodate the file's size
        if let Some(pos) = free_spans.iter().position(|span| span.length >= file.size) {
            let target_span = free_spans[pos]; // Get the target free span
            if file.start < target_span.start {
                continue; // Skip moving if the file is already before the target span
            }

            // Move the file to the start of the free span by setting the appropriate blocks
            for offset in 0..file.size {
                expanded[target_span.start + offset] = Some(file.file_id);
            }

            // Clear the original blocks where the file was located
            for offset in 0..file.size {
                expanded[file.start + offset] = None;
            }

            // Update the free spans based on how the file was moved
            if target_span.length == file.size {
                free_spans.remove(pos); // Remove the free span if it exactly fits the file
            } else {
                // Otherwise, adjust the start and length of the free span
                free_spans[pos].start += file.size;
                free_spans[pos].length -= file.size;
            }
        }
    }
}

pub fn part1(input: &str) -> String {
    let elasp = std::time::Instant::now();
    let segments = parse_input_in_segments(input); // Parse the input into segments
    let mut expanded = expand_segments(&segments); // Expand the segments into blocks
    move_files(&mut expanded);
    let checksum = calculate_checksum(&expanded);
    let elasp = elasp.elapsed();
    println!("Time: {:?}", elasp);
    checksum.to_string()
}

pub fn part2(input: &str) -> String {
    let elasp = std::time::Instant::now();
    let segments = parse_input_in_segments(input); // Parse the input into segments
    let mut expanded = expand_segments(&segments); // Expand the segments into blocks
    move_files_part2(&mut expanded);
    let checksum = calculate_checksum(&expanded);

    let elasp = elasp.elapsed();
    println!("Time: {:?}", elasp);
    checksum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402"; 

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "1928");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "2858");
    }
}
