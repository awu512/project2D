use crate::sprite::{Action, Character};
use crate::types::{Image, Rect, Vec2i};
use std::collections::hash_map::HashMap;
use std::rc::Rc;

const SPRITE_RECT_WIDTH: usize = 443;
const SPRITE_RECT_HEIGHT: usize = 401;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub struct Animation {
    pub frames: Vec<Rect>,
    pub frame_timings: Vec<usize>,
    pub loops: bool,
}

#[allow(dead_code)]
impl Animation {
    // Should hold some data...
    // Be used to decide what frame to use...
    // Could have a query function like current_frame(&self, start_time:usize, now:usize, speedup_factor:usize)
    // Or could be ticked in-place with a function like tick(&self)
    pub fn initial_frame(&self) -> Rect {
        self.frames[0]
    }

    pub fn current_frame(&self, start_time: usize, now: usize, speedup_factor: &usize) -> Rect {
        let frame_timing = (now - start_time) / speedup_factor;
        self.frames[frame_timing]
    }

    #[allow(dead_code)]
    fn is_finished(&self, start_time: usize, now: usize, speedup_factor: &usize) -> bool {
        // return true if the end time of this animation is passed.
        (now - start_time) / speedup_factor >= self.frames.len()
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub struct AnimationState {
    // Here you'll need to track how far along in the animation you are.
    // You can also choose to have an Rc<Animation> to refer to the animation in use.
    // But you could also decide to just pass around the animation and state together
    // where needed.
    pub start_time: usize,
    pub now: usize,
    pub action: Action,
    pub animation: Rc<Animation>,
}

impl AnimationState {
    #[allow(dead_code)]
    pub fn tick(&mut self, speedup_factor: &usize) -> Rect {
        self.now += 1;
        if self
            .animation
            .is_finished(self.start_time, self.now, speedup_factor)
            && self.animation.loops
        {
            self.now = 0;
        }
        self.animation
            .current_frame(self.start_time, self.now, speedup_factor)
    }
}

pub struct AnimationSet {
    pub character: Character, // dont need this
    pub image: Image,
    pub animations: HashMap<Action, Rc<Animation>>,
}

impl AnimationSet {
    pub fn get_animation(&self, action: Action) -> &Rc<Animation> {
        // let this return an AnimationState, clone
        self.animations.get(&action).unwrap()
    }

    pub fn play_animation(&self, action: Action) -> AnimationState {
        AnimationState {
            start_time: 0,
            now: 0,
            action: action,
            animation: self.animations.get(&action).unwrap().clone(),
        }
    }

    pub fn get_image(&self) -> &Image {
        &self.image
    }

    // pub fn get_animation_state(&self, action: Action) -> AnimationState {
    //     AnimationState {
    //         start_time: 0,
    //         now: 0,
    //         animation: self.animations.get(&action).unwrap(),
    //     }
    // }
}

// struct AnimQueue {
//     queue: Vec<(f32, AnimationState, bool)>,
// }

// impl AnimQueue {
//     #[allow(dead_code)]
//     fn push(&mut self, p: f32, anim: AnimationState, pause: bool, retrigger: bool) {
//         // If this is a retrigger, replace the old animation (if any)
//         // otherwise, leave the old animation alone!
//         let old_anim = anim.clone();
//         let to_insert = if let Some(found_pos) = self
//             .queue
//             .iter()
//             .position(|(qp, qanim, _)| qanim.animation == anim.animation)
//         {
//             let (_qp, qanim, _qpause) = self.queue.remove(found_pos);
//             if retrigger {
//                 (p, anim, pause)
//             } else {
//                 (p, qanim, pause)
//             }
//         } else {
//             (p, anim, pause)
//         };
//         // put highest priority thing at end
//         let pos = self
//             .queue
//             .iter()
//             .rposition(|(qp, _, _)| qp < &p)
//             .unwrap_or(0);
//         self.queue.insert(pos, (p, old_anim, pause));
//     }

//     #[allow(dead_code)]
//     fn tick(&mut self) {
//         let qlen = self.queue.len();
//         // tick possibly-paused non-current animations
//         if qlen > 1 {
//             for (_p, anim, pause) in self.queue.iter_mut().take(qlen - 2) {
//                 if !(*pause) {
//                     anim.tick();
//                 }
//             }
//         }
//         if let Some((_, active, _)) = self.queue.last() {
//             active.tick();
//         }
//         // Throw away finished animations
//         self.queue.retain(|(_p, anim, _)| !anim.is_finished());
//     }

//     // Got to return option here --- nothing to return if no animations in the queue!
//     #[allow(dead_code)]
//     fn current_frame(&self) -> Option<Rect> {
//         self.queue
//             .last()
//             .map(|(_, anim, _)| anim.animation.current_frame(0, 0, 0))
//     }
// }