use std::io::Result;
fn main() -> Result<()> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_well_known_types(true)
        .compile(
        &[
            "proto/github.com/containerd/containerd/api/types/platform.proto",
            "proto/github.com/containerd/containerd/api/types/metrics.proto",
            "proto/github.com/containerd/containerd/api/types/task/task.proto",
            "proto/github.com/containerd/containerd/api/types/descriptor.proto",
            "proto/github.com/containerd/containerd/api/types/mount.proto",
            "proto/github.com/containerd/containerd/api/events/namespace.proto",
            "proto/github.com/containerd/containerd/api/events/content.proto",
            "proto/github.com/containerd/containerd/api/events/task.proto",
            "proto/github.com/containerd/containerd/api/events/snapshot.proto",
            "proto/github.com/containerd/containerd/api/events/container.proto",
            "proto/github.com/containerd/containerd/api/events/image.proto",
            "proto/github.com/containerd/containerd/api/services/tasks/v1/tasks.proto",
            "proto/github.com/containerd/containerd/api/services/snapshots/v1/snapshots.proto",
            "proto/github.com/containerd/containerd/api/services/images/v1/images.proto",
            "proto/github.com/containerd/containerd/api/services/content/v1/content.proto",
            "proto/github.com/containerd/containerd/api/services/diff/v1/diff.proto",
            "proto/github.com/containerd/containerd/api/services/introspection/v1/introspection.proto",
            "proto/github.com/containerd/containerd/api/services/namespaces/v1/namespace.proto",
            "proto/github.com/containerd/containerd/api/services/leases/v1/leases.proto",
            "proto/github.com/containerd/containerd/api/services/version/v1/version.proto",
            "proto/github.com/containerd/containerd/api/services/containers/v1/containers.proto",
            "proto/github.com/containerd/containerd/api/services/ttrpc/events/v1/events.proto",
            "proto/github.com/containerd/containerd/api/services/events/v1/events.proto",
            "proto/github.com/containerd/containerd/protobuf/plugin/fieldpath.proto",
        ],
        &[
            "proto/",
            "proto/github.com/containerd/containerd/vendor/github.com/gogo/protobuf/",
            "proto/github.com/containerd/containerd/vendor/github.com/gogo/googleapis/"
        ],
    )?;
    Ok(())
}
