// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Logging* crate version *1.0.0+20161212*, where *20161212* is the exact revision of the *logging:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.0*.
//! 
//! Everything else about the *Logging* *v2* API can be found at the
//! [official documentation site](https://cloud.google.com/logging/docs/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/logging2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Logging.html) ... 
//! 
//! * billing accounts
//!  * [*logs delete*](struct.BillingAccountLogDeleteCall.html), [*logs list*](struct.BillingAccountLogListCall.html), [*sinks create*](struct.BillingAccountSinkCreateCall.html), [*sinks delete*](struct.BillingAccountSinkDeleteCall.html), [*sinks get*](struct.BillingAccountSinkGetCall.html), [*sinks list*](struct.BillingAccountSinkListCall.html) and [*sinks update*](struct.BillingAccountSinkUpdateCall.html)
//! * entries
//!  * [*list*](struct.EntryListCall.html) and [*write*](struct.EntryWriteCall.html)
//! * folders
//!  * [*logs delete*](struct.FolderLogDeleteCall.html), [*logs list*](struct.FolderLogListCall.html), [*sinks create*](struct.FolderSinkCreateCall.html), [*sinks delete*](struct.FolderSinkDeleteCall.html), [*sinks get*](struct.FolderSinkGetCall.html), [*sinks list*](struct.FolderSinkListCall.html) and [*sinks update*](struct.FolderSinkUpdateCall.html)
//! * [monitored resource descriptors](struct.MonitoredResourceDescriptor.html)
//!  * [*list*](struct.MonitoredResourceDescriptorListCall.html)
//! * organizations
//!  * [*logs delete*](struct.OrganizationLogDeleteCall.html), [*logs list*](struct.OrganizationLogListCall.html), [*sinks create*](struct.OrganizationSinkCreateCall.html), [*sinks delete*](struct.OrganizationSinkDeleteCall.html), [*sinks get*](struct.OrganizationSinkGetCall.html), [*sinks list*](struct.OrganizationSinkListCall.html) and [*sinks update*](struct.OrganizationSinkUpdateCall.html)
//! * projects
//!  * [*logs delete*](struct.ProjectLogDeleteCall.html), [*logs list*](struct.ProjectLogListCall.html), [*metrics create*](struct.ProjectMetricCreateCall.html), [*metrics delete*](struct.ProjectMetricDeleteCall.html), [*metrics get*](struct.ProjectMetricGetCall.html), [*metrics list*](struct.ProjectMetricListCall.html), [*metrics update*](struct.ProjectMetricUpdateCall.html), [*sinks create*](struct.ProjectSinkCreateCall.html), [*sinks delete*](struct.ProjectSinkDeleteCall.html), [*sinks get*](struct.ProjectSinkGetCall.html), [*sinks list*](struct.ProjectSinkListCall.html) and [*sinks update*](struct.ProjectSinkUpdateCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Logging.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.projects().sinks_update(...).doit()
//! let r = hub.billing_accounts().sinks_get(...).doit()
//! let r = hub.billing_accounts().sinks_create(...).doit()
//! let r = hub.folders().sinks_create(...).doit()
//! let r = hub.organizations().sinks_create(...).doit()
//! let r = hub.organizations().sinks_update(...).doit()
//! let r = hub.billing_accounts().sinks_update(...).doit()
//! let r = hub.organizations().sinks_get(...).doit()
//! let r = hub.projects().sinks_create(...).doit()
//! let r = hub.folders().sinks_update(...).doit()
//! let r = hub.projects().sinks_get(...).doit()
//! let r = hub.folders().sinks_get(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-logging2 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_logging2 as logging2;
//! use logging2::LogSink;
//! use logging2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use logging2::Logging;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Logging::new(hyper::Client::new(), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = LogSink::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.projects().sinks_update(req, "sinkName")
//!              .unique_writer_identity(false)
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![cfg_attr(feature = "nightly", feature(proc_macro))]
#![allow(unused_imports, unused_mut, dead_code)]

#[cfg(feature = "nightly")]
include!("lib.rs.in");

#[cfg(feature = "with-serde-codegen")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));