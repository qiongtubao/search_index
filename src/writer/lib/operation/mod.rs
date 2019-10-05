use crate::writer::lib::operation::add::AddOperation;

pub mod add;
use crossbeam::channel;
use smallvec::SmallVec;
//啥用的啊  封装vec?
pub(crate) type OperationGroup = SmallVec<[AddOperation; 4]>;
pub(crate) type OperationSender = channel::Sender<OperationGroup>;
pub(crate) type OperationReceiver = channel::Receiver<OperationGroup>;
