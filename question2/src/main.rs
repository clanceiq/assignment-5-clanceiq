pub struct L{
    x: usize,
    y: usize,
}

// TODO: add iterators
pub fn foo(text: &str, string: &str) -> Vec<L> {
    text.lines() // split text into lines
        .enumerate() // gives each line a number (x)
        .flat_map(|(x, line)| { //
            line.match_indices(string) // find all occurences of "string" in the line
                .map(move |(y, _)| L { x, y }) // for each occurence, turn into an L struct
        })
        .collect() // creates a vector of all the L structs
}

fn main() {
    let results = foo("Shall I compare thee to a summer's day?
    Thou art more lovely and more temperate:
    Rough winds do shake the darling buds of May,
    And summer's lease hath all too short a date:
    Sometimes too hot the eye of heaven shines,
    And too often is his gold complexion dimm'd:
    And every fair from fair sometimes declines,
    By chance or natures changing course untrimm'd;
    By thy eternal summer shall not fade,
    Nor lose possession of that fair thou owest;
    Nor shall Death brag thou wander'st in his shade,
    When in eternal lines to time thou growest:
    So long as men can breathe or eyes can see,
    So long lives this and this gives life to thee.", "the");
    for x in results {println!("x : {}, y : {}", x.x, x.y);}
}
