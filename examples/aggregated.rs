use w3f_bls::{distinct::DistinctMessages, Keypair, Message, Signed, ZBLS};

fn main() {
    let mut keypairs = [
        Keypair::<ZBLS>::generate(::rand::thread_rng()),
        Keypair::<ZBLS>::generate(::rand::thread_rng()),
    ];
    let msgs = [
        "The ships",
        "hung in the sky",
        "in much the same way",
        "that bricks don’t.",
    ]
    .iter()
    .map(|m| Message::new(b"Some context", m.as_bytes()))
    .collect::<Vec<_>>();
    let sigs = msgs
        .iter()
        .zip(keypairs.iter_mut())
        .map(|(m, k)| k.signed_message(m))
        .collect::<Vec<_>>();

    let mut dms = sigs
        .iter()
        .try_fold(DistinctMessages::<ZBLS>::new(), |dm, sig| dm.add(sig))
        .unwrap();
    let signature = <&DistinctMessages<ZBLS> as Signed>::signature(&&dms);

    let publickeys = keypairs.iter().map(|k| k.public).collect::<Vec<_>>();
    let mut dms = msgs
        .into_iter()
        .zip(publickeys)
        .try_fold(
            DistinctMessages::<ZBLS>::new(),
            |dm, (message, publickey)| dm.add_message_n_publickey(message, publickey),
        )
        .unwrap();
    dms.add_signature(&signature);
    assert!(dms.verify())
}
