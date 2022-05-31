mod garden;
use garden::plant_type::vegetables::Asparagus;
use garden::plant_type::fruits::Bannana;


fn main() {
    let vege = Asparagus {};
    println!("I'm growing. {:?}", vege);

    let fruite = Bannana {};
    println!("I'm eatting. {:?}", fruite);
}
