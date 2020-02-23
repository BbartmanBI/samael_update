use crate::metadata::{
    AffiliationDescriptor, AttributeAuthorityDescriptors, AuthnAuthorityDescriptors, ContactPerson,
    IdpSsoDescriptor, Organization, PdpDescriptors, RoleDescriptor, SpSsoDescriptor,
};
use crate::signature::Signature;
use chrono::prelude::*;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Writer;
use serde::Deserialize;
use std::io::Cursor;

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "md:EntityDescriptor")]
pub struct EntityDescriptor {
    #[serde(rename = "entityID")]
    pub entity_id: Option<String>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "ds:Signature")]
    pub signature: Option<Signature>,
    #[serde(rename = "validUntil")]
    pub valid_until: Option<DateTime<Utc>>,
    #[serde(rename = "cacheDuration")]
    pub cache_duration: Option<String>,
    #[serde(rename = "md:RoleDescriptor")]
    pub role_descriptors: Option<Vec<RoleDescriptor>>,
    #[serde(rename = "md:IDPSSODescriptor")]
    pub idp_sso_descriptors: Option<Vec<IdpSsoDescriptor>>,
    #[serde(rename = "md:SPSSODescriptor")]
    pub sp_sso_descriptors: Option<Vec<SpSsoDescriptor>>,
    #[serde(rename = "md:AuthnAuthorityDescriptor")]
    pub authn_authority_descriptors: Option<Vec<AuthnAuthorityDescriptors>>,
    #[serde(rename = "md:AttributeAuthorityDescriptor")]
    pub attribute_authority_descriptors: Option<Vec<AttributeAuthorityDescriptors>>,
    #[serde(rename = "md:PDPDescriptor")]
    pub pdp_descriptors: Option<Vec<PdpDescriptors>>,
    #[serde(rename = "md:AffiliationDescriptor")]
    pub affiliation_descriptors: Option<AffiliationDescriptor>,
    #[serde(rename = "md:ContactPerson")]
    pub contact_person: Option<ContactPerson>,
    #[serde(rename = "md:Organization")]
    pub organization: Option<Organization>,
}

impl EntityDescriptor {
    pub fn to_xml(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut write_buf = Vec::new();
        let mut writer = Writer::new(Cursor::new(&mut write_buf));
        let root_name = "md:EntityDescriptor";
        let mut root = BytesStart::borrowed(root_name.as_bytes(), root_name.len());
        if let Some(entity_id) = &self.entity_id {
            root.push_attribute(("entityID", entity_id.as_ref()))
        }
        if let Some(valid_until) = &self.valid_until {
            root.push_attribute((
                "validUntil",
                valid_until
                    .to_rfc3339_opts(SecondsFormat::Secs, true)
                    .as_ref(),
            ))
        }
        root.push_attribute(("xmlns:md", "urn:oasis:names:tc:SAML:2.0:metadata"));
        root.push_attribute(("xmlns:saml", "urn:oasis:names:tc:SAML:2.0:assertion"));
        root.push_attribute(("xmlns:mdrpi", "urn:oasis:names:tc:SAML:metadata:rpi"));
        root.push_attribute(("xmlns:mdattr", "urn:oasis:names:tc:SAML:metadata:attribute"));
        root.push_attribute(("xmlns:mdui", "urn:oasis:names:tc:SAML:metadata:ui"));
        root.push_attribute((
            "xmlns:idpdisc",
            "urn:oasis:names:tc:SAML:profiles:SSO:idp-discovery-protocol",
        ));
        root.push_attribute(("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"));
        writer.write_event(Event::Start(root))?;
        if let Some(sp_sso_descriptors) = &self.sp_sso_descriptors {
            for descriptor in sp_sso_descriptors {
                writer.write(descriptor.to_xml()?.as_bytes())?;
            }
        }
        if let Some(organization) = &self.organization {
            writer.write(organization.to_xml()?.as_bytes())?;
        }
        if let Some(contact_person) = &self.contact_person {
            writer.write(contact_person.to_xml()?.as_bytes())?;
        }
        writer.write_event(Event::End(BytesEnd::borrowed(root_name.as_bytes())))?;

        Ok(String::from_utf8(write_buf)?)
    }
}