
/**
 * Routes:
 * Authentication using Basic header
 * 
 * Endpoint:
 * /testcases
 * POST: New testcase
 * GET: Get all testcases
 * /testcases?assg_num=<num>&q_num=<num>
 * GET: Get filtered testcases
 * /testcases/<id>
 * DELETE: If it is posted by same user, delete
 * PUT: Update testcase 
 * 
 * /submissions/
 * POST: Add submission for a testcase
 * {testcase_id: ..., content: ...}
 * 
 * /submissions?sorted_by_majority
 * /submissions?user=<user>
 * GET: Get submissions
 * 
 * /change_password
 * POST: Update password
 * 
 * 
 */

fn main() {
    println!("Hello, world!");
}

fn setup_tables() {
    // TODO
}

fn authenticate(user: &str, password: &str) {
    // TODO
}