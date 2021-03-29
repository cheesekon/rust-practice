
// let arr1: [u32; 5] = [999999999, 999999999, 999999999, 999999999, 999999999];
// let arr2: [u32; 5] = [1, 2, 3, 4, 5];

// println!("The arr1's summation result is {:?}", summation(&arr1));
// println!("The arr2's summation result is {:?}", summation(&arr2));

pub fn summation(arr: &[u32]) -> Option<u32> {
  let mut res: Option<u32> = Some(0);

  for &i in arr {
      res = res.unwrap().checked_add(i);
  }
  
  res
}


// let yellow: TrafficLight = TrafficLight::Yellow;
// let red: TrafficLight = TrafficLight::Red;
// let green: TrafficLight = TrafficLight::Green;

// println!("Yellow is {}", yellow.time());
// println!("Red is {}", red.time());
// println!("Green is {}", green.time());

trait TrafficLightTrait {
  fn time(&self) -> u8;
}

enum TrafficLight {
  Red,
  Green,
  Yellow
}

impl TrafficLightTrait for TrafficLight {
  fn time(&self) -> u8 {
      match self {
          TrafficLight::Red => 10,
          TrafficLight::Green => 15,
          TrafficLight::Yellow => 5
      }
  }
}