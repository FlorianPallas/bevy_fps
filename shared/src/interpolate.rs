use bevy::prelude::*;

pub struct InterpolatePlugin;

impl Plugin for InterpolatePlugin {
    fn build(&self, app: &mut App) {
        app.register_interpolate::<Transform, InterpolateTransform>()
            .register_interpolate::<Transform, InterpolateTranslation>()
            .register_interpolate::<Transform, InterpolateRotation>()
            .register_interpolate::<Transform, InterpolateScale>();
    }
}

pub trait InterpolateAppExt {
    fn register_interpolate<T: Component, M: Component + Interpolate<T>>(&mut self) -> &mut Self;
}

impl InterpolateAppExt for App {
    fn register_interpolate<T: Component, M: Component + Interpolate<T>>(&mut self) -> &mut Self {
        self.add_systems(FixedPreUpdate, switch::<T, M>);
        self.add_systems(FixedPostUpdate, target::<T, M>);
        self.add_systems(Update, interpolate::<T, M>)
    }
}

#[derive(Component, Default)]
pub struct InterpolateBuffer<T> {
    start: Option<T>,
    end: Option<T>,
}

pub trait Interpolate<T>: Component {
    type State;
    fn get_buffer(&self) -> &InterpolateBuffer<Self::State>;
    fn get_buffer_mut(&mut self) -> &mut InterpolateBuffer<Self::State>;
    fn get_state(target: &T) -> Self::State;
    fn set_state(target: &mut T, state: Self::State);
    fn interpolate(start: &Self::State, end: &Self::State, weight: f32) -> Self::State;
}

pub fn target<T: Component, M: Interpolate<T>>(mut q: Query<(&T, &mut M)>) {
    for (target, mut marker) in q.iter_mut() {
        marker.get_buffer_mut().end = Some(M::get_state(target));
    }
}

pub fn interpolate<T: Component, M: Interpolate<T>>(
    mut q: Query<(&mut T, &M)>,
    time: Res<Time<Fixed>>,
) {
    let scale = time.overstep_fraction() % 1.0;
    for (mut target, marker) in q.iter_mut() {
        let buffer = marker.get_buffer();
        if let (Some(from), Some(to)) = (buffer.start.as_ref(), buffer.end.as_ref()) {
            M::set_state(&mut target, M::interpolate(from, to, scale));
        }
    }
}

pub fn switch<T: Component, M: Interpolate<T>>(mut q: Query<(&mut T, &mut M)>) {
    for (mut target, mut buffer) in q.iter_mut() {
        let buffer = buffer.get_buffer_mut();
        if let Some(to) = buffer.end.take() {
            M::set_state(&mut target, to);
            buffer.start = Some(M::get_state(&target));
        }
    }
}

#[derive(Component, Default)]
pub struct InterpolateTranslation(InterpolateBuffer<Vec3>);

#[derive(Component, Default)]
pub struct InterpolateTransform(InterpolateBuffer<Transform>);

#[derive(Component, Default)]
pub struct InterpolateRotation(InterpolateBuffer<Quat>);

#[derive(Component, Default)]
pub struct InterpolateScale(InterpolateBuffer<Vec3>);

impl Interpolate<Transform> for InterpolateTransform {
    type State = Transform;

    fn get_buffer(&self) -> &InterpolateBuffer<Self::State> {
        &self.0
    }

    fn get_buffer_mut(&mut self) -> &mut InterpolateBuffer<Self::State> {
        &mut self.0
    }

    fn get_state(target: &Transform) -> Self::State {
        *target
    }

    fn set_state(target: &mut Transform, state: Self::State) {
        *target = state;
    }

    fn interpolate(from: &Self::State, to: &Self::State, scale: f32) -> Self::State {
        Transform::default()
            .with_translation(from.translation.lerp(to.translation, scale))
            .with_rotation(from.rotation.slerp(to.rotation, scale))
            .with_scale(from.scale.lerp(to.scale, scale))
    }
}

impl Interpolate<Transform> for InterpolateRotation {
    type State = Quat;

    fn get_buffer(&self) -> &InterpolateBuffer<Self::State> {
        &self.0
    }

    fn get_buffer_mut(&mut self) -> &mut InterpolateBuffer<Self::State> {
        &mut self.0
    }

    fn get_state(target: &Transform) -> Self::State {
        target.rotation
    }

    fn set_state(target: &mut Transform, state: Self::State) {
        target.rotation = state;
    }

    fn interpolate(from: &Self::State, to: &Self::State, scale: f32) -> Self::State {
        from.slerp(*to, scale)
    }
}

impl Interpolate<Transform> for InterpolateTranslation {
    type State = Vec3;

    fn get_buffer(&self) -> &InterpolateBuffer<Self::State> {
        &self.0
    }

    fn get_buffer_mut(&mut self) -> &mut InterpolateBuffer<Self::State> {
        &mut self.0
    }

    fn get_state(target: &Transform) -> Self::State {
        target.translation
    }

    fn set_state(target: &mut Transform, state: Self::State) {
        target.translation = state;
    }

    fn interpolate(from: &Self::State, to: &Self::State, scale: f32) -> Self::State {
        from.lerp(*to, scale)
    }
}

impl Interpolate<Transform> for InterpolateScale {
    type State = Vec3;

    fn get_buffer(&self) -> &InterpolateBuffer<Self::State> {
        &self.0
    }

    fn get_buffer_mut(&mut self) -> &mut InterpolateBuffer<Self::State> {
        &mut self.0
    }

    fn get_state(target: &Transform) -> Self::State {
        target.scale
    }

    fn set_state(target: &mut Transform, state: Self::State) {
        target.scale = state
    }

    fn interpolate(from: &Self::State, to: &Self::State, scale: f32) -> Self::State {
        from.lerp(*to, scale)
    }
}
