use yew::prelude::*;

enum Plant {
    Tomato,
    Salad,
    Potato
}
trait PlantCard {
    fn create_card(plant: Plant) -> Card;
}
impl PlantCard for Plant {
    fn create_card(plant: Plant) -> Card {
        match plant{
            Plant::Potato => Card::new(String::from("Potato"), String::from("Icky Potato")),
            Plant::Salad => Card::new(String::from("Salad"), String::from("Green ball")),
            Plant::Tomato => Card::new(String::from("Tomato"), String::from("Red ball"))
        }
    }
}

struct Card {
    name: String,
    desc: String
}
impl Card {
    fn new(name: String, desc: String) -> Self {
        Self {
            name: name,
            desc: desc
        }
    }
}
#[function_component(App)]
fn app() -> Html {
    html! {
        <>   
            <header>{"PlantCards"}</header>
            <div class={"cards"}>
            {get_cards()}
            </div>
        </>
    }
}
fn get_cards() -> Html {
    let pot: Plant = Plant::Potato;
    let sal: Plant = Plant::Salad;
    let tot: Plant = Plant::Tomato;

    let cards: [Card; 3] = [
        Plant::create_card(pot),
        Plant::create_card(tot),
        Plant::create_card(sal)
    ];
    
    cards.iter().map(| card |{
        let name = card.name.clone();
        let desc = card.desc.clone();

        html! {
            <div class={"card"}>
                <h2>{name}</h2>
                <p>{desc}</p>
            </div> 
        }
    }).collect()
}
fn main() {
    yew::Renderer::<App>::new().render();
}