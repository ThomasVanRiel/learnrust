fn main() {
    let poem = "I have a little shadow that goes in and out with me,
And what can be the use of him is more than I can see.
He is very, very like me from the heels up to the head;
And I see him jump before me, when I jump into my bed.";

    let query = "me";

    for line in poem.lines() {
        if line.contains(query) {
            println!("{line}");
        }
    }
}
