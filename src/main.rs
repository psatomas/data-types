fn main() {
    let user_has_paid_for_subscription = true;
    let user_is_admin = true;
    let user_can_see_premium_experience = user_has_paid_for_subscription || user_is_admin;
    println!("Can this user see my site?{user_can_see_premium_experience}");
}
