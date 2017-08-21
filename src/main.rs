
extern crate openssl;
extern crate kafka;

// Standard libs
use std::env;

// Kafka dependencies.
use openssl::ssl::{ SslConnectorBuilder, SslMethod, SSL_VERIFY_PEER };
use openssl::x509::X509_FILETYPE_PEM;
use kafka::client::{ KafkaClient, SecurityConfig };
use kafka::producer::{ Producer };
use kafka::consumer::{ Consumer };

// Own dependencies
mod config;
mod error;
mod messages;

fn main() {

	let mut client: KafkaClient = create_client();

	let mut producer = Producer::from_client(client)
		.with_ack_timeout(Duration::from_secs(1))
		.with_required_acks(RequiredAcks::One)
		.create()
		.unwrap();

	let mut consumer = Consumer::from_client(client)
		.with_topic_partitions("identity".to_owned(), &[0, 1])
		.with_fallback_offset(FetchOffset::Earliest)
		.with_group("identity".to_owned())
		.with_offset_storage(GroupOffsetStorage::Kafka)
		.create()
		.unwrap();

	loop {

		// Grab the next available MessageSet
		for ms in consumer.poll().unwrap().iter() {

			// Run through each of the messages.
			for m in ms.messages() {
				// TODO: parse messages.
			}

			consumer.consume_messageset(ms);

		}

		consumer.commit_consumed().unwrap();

	}

}

fn create_client() {

	let kafka_hosts = config::KAFKA_HOSTS
		.unwrap_or_else(|| panic!("Can't run the service without hosts."))
		.split(",")
		.map(|s| s.to_string())
		.collect();

	let kafka_ssl_key  = config::KAFKA_SSL_KEY;
	let kafka_ssl_cert = config::KAFKA_SSL_CERT;

	if kafka_ssl_key.is_none() && kafka_ssl_cert.is_none() {
		return KafkaClient::new(hosts);
	}

	if kafka_ssl_key.is_some() && kafka_ssl_cert.is_none() {
		panic!("SSL key provided, but not certificate!")
	}

	if kafka_ssl_cert.is_some() && kafka_ssl_key.is_none() {
		panic!("SSL certificate provided, but not key!")
	}

	// check to see if the key and cert are files, and set builder content appropriately.
	let mut builder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();

	let mut ctx = builder.builder_mut();
	ctx.set_cipher_list("DEFAULT").unwrap();
	ctx.set_default_verify_paths().unwrap();
	ctx.set_verify(SSL_VERIFY_PEER);

	// TODO: if cert is a filename
	ctx.set_certificate_file(&cert, X509_FILETYPE_PEM).unwrap();

	// TODO: if key is a filename
	ctx.set_private_key_file(&key, X509_FILETYPE_PEM).unwrap();

	return KafkaClient::new_secure(hosts, SecurityConfig::new(builder.build()));

}
