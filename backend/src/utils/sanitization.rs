pub fn sanitize_email(email: &str) -> String {
    let mut email = email.trim().to_lowercase();
    
    // List of known TLDs to check for
    // If the email ends with any of these followed by other characters, we chop it off.
    let known_tlds = [
        ".com", ".nl", ".net", ".org", ".be", ".eu", ".de", ".uk", ".edu", ".gov"
    ];

    for tld in known_tlds {
        if let Some(idx) = email.rfind(tld) {
            // Check if there is anything AFTER the TLD
            let end_of_tld = idx + tld.len();
            if end_of_tld < email.len() {
                // We found a TLD, but there is extra stuff afterwards.
                // Truncate it!
                // Safety check: ensure the TLD matches valid domain characters before it
                // e.g. avoid false positives if a username has .com in it? 
                // But emails usually only have one relevant TLD at the end of the domain.
                // We assume the verified TLD is part of the domain.
                
                // Let's verify this is the domain part
                if let Some(at_idx) = email.find('@') {
                    if idx > at_idx {
                         match email.get(0..end_of_tld) {
                             Some(clean_slice) => return clean_slice.to_string(),
                             None => {} // Should not happen
                         }
                    }
                }
            }
        }
    }
    
    // Also strip trailing dots just in case
    if email.ends_with('.') {
        email.pop();
    }

    email
}
