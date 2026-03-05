use wm_game::state::GameState;

use crate::events::UiEvent;
use crate::screen::{Screen, ScreenAction, ScreenId};
use crate::widget::{Widget, WidgetBase, WidgetStore, WidgetId};
use crate::widget::button::ButtonWidget;
use crate::widget::listbox::ListBoxWidget;
use crate::widget::text_item::TextItemWidget;

#[derive(Debug)]
pub struct TurnSummaryScreen {
    events: Vec<String>,
    event_list_id: WidgetId,
    summary_id: WidgetId,
    back_id: WidgetId,
}

impl TurnSummaryScreen {
    pub fn with_events(events: Vec<String>) -> Self {
        Self {
            events,
            event_list_id: 0,
            summary_id: 0,
            back_id: 0,
        }
    }
}

impl Screen for TurnSummaryScreen {
    fn id(&self) -> ScreenId { "turn_summary" }

    fn init(&mut self, widgets: &mut WidgetStore, state: &mut GameState) {
        widgets.clear();

        // Event list
        let id = widgets.allocate_id();
        let base = WidgetBase::new(id, "EventList", 30, 30, 740, 450);
        let mut lb = ListBoxWidget {
            base,
            items: Vec::new(),
            columns: Vec::new(),
            multi_select: false,
            show_headers: false,
            header_dividers: false,
            header_clicks_sort: false,
            scroll_position: 0,
            sorted_column: String::new(),
            sorted_descending: false,
            border_size: 1,
            element_height: 20,
        };
        for (i, evt) in self.events.iter().enumerate() {
            lb.add_element(i as i32, evt);
        }
        self.event_list_id = widgets.add("EventList", Widget::ListBox(lb));

        // Summary text
        let id2 = widgets.allocate_id();
        let base2 = WidgetBase::new(id2, "Summary", 30, 490, 740, 60);
        let summary_text = format!(
            "Week {} complete  |  Gold: {:.0}  |  Girls: {}  |  Brothels: {}",
            state.week, state.gold.cash_on_hand, state.girls.count(), state.brothels.num_brothels(),
        );
        let ti = TextItemWidget {
            base: base2,
            text: summary_text,
            font_size: 14,
            scroll_offset: 0,
            total_height: 0,
        };
        self.summary_id = widgets.add("Summary", Widget::TextItem(ti));

        // Back button
        let id3 = widgets.allocate_id();
        let base3 = WidgetBase::new(id3, "BackButton", 350, 560, 100, 30);
        let btn = ButtonWidget {
            base: base3,
            image_off: "BackButtonOff.png".into(),
            image_on: "BackButtonOn.png".into(),
            image_disabled: "BackButtonDisabled.png".into(),
            transparency: true,
            scale: true,
            pressed: false,
        };
        self.back_id = widgets.add("BackButton", Widget::Button(btn));
    }

    fn process(&mut self, _widgets: &mut WidgetStore, _state: &mut GameState) -> ScreenAction {
        ScreenAction::None
    }

    fn on_event(&mut self, event: UiEvent, widgets: &mut WidgetStore, _state: &mut GameState) -> ScreenAction {
        if let UiEvent::MouseClick { x, y } = event {
            if let Some(Widget::Button(b)) = widgets.get(self.back_id) {
                if b.base.is_over(x, y) { return ScreenAction::Pop; }
            }
            // Scroll event list
            if let Some(Widget::ListBox(lb)) = widgets.get_mut(self.event_list_id) {
                if lb.base.is_over(x, y) {
                    lb.handle_click(x, y);
                }
            }
        }
        ScreenAction::None
    }
}
