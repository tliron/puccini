use super::tosca::*;

use {compris::annotate::*, kutil::std::error::*};

//
// ToscaErrorReceiverRef
//

/// Common reference type for [ErrorReceiver] of [ToscaError].
pub type ToscaErrorReceiverRef<'own> = ErrorReceiverRef<'own, ToscaError<WithAnnotations>>;
