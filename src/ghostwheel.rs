// -----------------
// Model
trait Entity {
  fn get_key() -> u64
}

struct Schema {
  entities: [Entity]
}

struct Ghostwheel {
  schema: Schema
}

// -----------------
// Example
soldier = wheel_entity! {
  struct Soldier : Entity  {
    name: str,
    age: u8
    rank: u8
  }
}
wheel_key!(soldier, |s| s.name)
wheel_formula!(soldier, |s| s.rank = s.age + len(s.name))

squad = wheel_entity! {
  struct Squad : Entity {
    title: str,
    size: u8
    soldiers: [Soldier]
  }
}
wheel_key!(squad, |s| s.title)
wheel_formula!(squad, |s| s.size = )
wheel_formula!(squad, |s| s.size = len(soldiers))

pub fn run_ghostwheel() {
  let wheel = Ghostwheel {
    schema: Schema::new([])
  }

  let a = A {}
  wheel.update(a)
  let b = 



}