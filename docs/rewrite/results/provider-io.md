# Provider I/O

Status: implementation and focused tests ready for integration-owner verification. Not an integrated parity or release completion claim.

## Behavior

`core/app/src/network/` provides the shared, cancellation-aware provider transport used by catalog and content callers. Each request supplies exact permitted origins, an explicit redirect count, independent encoded/decoded byte bounds, and either a validated SHA-1/SHA-256/SHA-512 checksum or a named reason that the provider supplies no checksum. HTTPS is mandatory outside a test-only literal-loopback seam. Download completion returns bytes and evidence only after full transfer, decoding, size validation and integrity verification; it grants no filesystem publication authority.

GET redirects are admitted against the caller's original origin policy on every hop. Bounded JSON POST queries only replay on same-origin 307/308 redirects. Error messages and returned origin evidence omit provider paths, queries and response bodies. The client explicitly disables automatic retries, including reqwest's protocol-level retries.

Identity, gzip and zlib-wrapped deflate are supported. Both gzip members and the complete zlib stream contribute to integrity and size checks. Truncated, corrupt and trailing compressed representations fail. Incremental zlib decoding uses `FlushDecompress::None` because its fixed output chunk cannot promise room for all remaining data; completion still requires a verified `StreamEnd` and consumption of the complete encoded representation. Decoding and hashing yield between bounded chunks for cancellation and whole-operation timeouts.

`pinned_public_transfer_client` and `managed_transfer_retry_policy` adapt the retained managed-file transfer leaf for Performance's real artifact resolver. They preserve public address pinning, finite DNS/connect/read/request timeouts and the existing transient-failure retry delays. The origin must match the supplied URL before resolution. Literal IPv4/IPv6 addresses need no DNS; resolved addresses are deduplicated and bounded before public-address admission. The retained leaf owns transfer staging, digest/size validation, settlement, proxy disabling and redirect confinement.

## Verification handoff

38 focused tests are supplied: 31 policy and controlled raw-HTTP fixture tests, 2 deterministic cancellation tests during decoder/hash work, and 5 managed-transfer admission tests. Cases cover exact origins and ports, URL rejection, finite policies, independent checksum vectors, explicit unhashed evidence, encoded/decoded bounds, unfinished transfer framing, partial responses, status failures, gzip members, zlib bodies larger than a decoder chunk, corrupted/truncated/trailing compressed bodies, unsupported encodings, redirects and POST disclosure prevention, bounded request bodies, pre-cancellation, stalled network cancellation and read/whole-operation timeouts. Negative redirect and admission fixtures verify no connection reaches the denied destination. Managed-transfer tests cover public literal clients without network transfer, private/mixed address rejection, origin mismatch, finite address sets/timeouts and retry classification.

Integration owner command:

```sh
cargo test -p axial-app network:: --lib
```

Run through the shared Cargo writer lease and capture output to a log; inspect its tail. This feature owner did not run Cargo/build commands. Catalog/content consumer tests and the assembled application's real consumer flows must also pass before the package is marked integrated. Current tests use controlled loopback HTTP, not live provider availability or a production TLS endpoint. No UI or legacy source changed.

## Boundaries

The API deliberately buffers bounded bodies (maximum 512 MiB for each representation); streaming filesystem publication and artifact settlement remain consumer/managed-file responsibilities. A provider checksum verifies agreement with provider metadata, not publisher authenticity. Source-specific caching, retry policy, authentication and filesystem authority remain feature-owned. No package-completion registry was changed by this owner.
