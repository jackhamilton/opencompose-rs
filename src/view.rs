pub struct View<State: LifecycleState> {
    _marker: std::marker::PhantomData<State>
}

impl View<Initialized> {

}

impl View<Displayed> {

}

impl View<Disposed> {}

pub trait LifecycleState {}
struct Initialized;
struct Displayed;
struct Disposed;
impl LifecycleState for Initialized {}
impl LifecycleState for Displayed {}
impl LifecycleState for Disposed {}
