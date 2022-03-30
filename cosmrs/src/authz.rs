use crate::{proto, tx::Msg, AccountId, Coin, ErrorReport, Result};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgRevoke {
    pub granter: AccountId,

    pub grantee: AccountId,

    pub msg_type_url: String,
}

impl Msg for MsgRevoke {
    type Proto = proto::cosmos::authz::v1beta1::MsgRevoke;
}

impl TryFrom<proto::cosmos::authz::v1beta1::MsgRevoke>
    for MsgRevoke
{
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cosmos::authz::v1beta1::MsgRevoke,
    ) -> Result<MsgRevoke> {
        MsgRevoke::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::authz::v1beta1::MsgRevoke>
    for MsgRevoke
{
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cosmos::authz::v1beta1::MsgRevoke,
    ) -> Result<MsgRevoke> {
        Ok(MsgRevoke {
            granter: proto.granter.parse()?,
            grantee: proto.grantee.parse()?,
            msg_type_url: proto.msg_type_url.parse()?
        })
    }
}

impl From<MsgRevoke> for proto::cosmos::authz::v1beta1::MsgRevoke {
    fn from(
        coin: MsgRevoke,
    ) -> proto::cosmos::authz::v1beta1::MsgRevoke {
        proto::cosmos::authz::v1beta1::MsgRevoke::from(&coin)
    }
}

impl From<&MsgRevoke> for proto::cosmos::authz::v1beta1::MsgRevoke {
    fn from(
        msg: &MsgRevoke,
    ) -> proto::cosmos::authz::v1beta1::MsgRevoke {
        proto::cosmos::authz::v1beta1::MsgRevoke {
            granter: msg.granter.to_string(),
            grantee: msg.grantee.to_string(),
            msg_type_url: msg.msg_type_url.to_string()
        }
    }
}