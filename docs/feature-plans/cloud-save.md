# Cloud Save

Synchronize save data across devices.

## Overview

Cloud save allows players to continue their progress on different devices by storing save data in the cloud.

## Features

- Automatic sync on save/load
- Manual sync option
- Conflict resolution (newer save wins or player choice)
- Offline fallback to local saves

## Implementation Considerations

### Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Device A  │────▶│  Cloud API  │◀────│   Device B  │
│  (Local)    │     │  (Server)   │     │  (Local)    │
└─────────────┘     └─────────────┘     └─────────────┘
```

### Sync Flow

1. On save: Upload to cloud if connected
2. On load: Check cloud for newer save
3. On conflict: Prompt user or use newest
4. Offline: Queue sync for later

### Backend Options

| Provider | Pros | Cons |
|----------|------|------|
| Steam Cloud | Built-in, free | Steam only |
| Custom server | Full control | Maintenance cost |
| Firebase | Easy setup | Google dependency |
| Supabase | Open source | Self-hosting needed |

### Data Format

```json
{
  "version": "1.0.0",
  "timestamp": "2025-01-15T12:00:00Z",
  "device_id": "abc123",
  "saves": [
    {
      "slot": 1,
      "data": "base64-encoded-save-data",
      "checksum": "sha256-hash"
    }
  ]
}
```

### Security

- End-to-end encryption
- User authentication
- Data validation on upload/download

## Platform Considerations

### Native

- Steam Cloud API integration
- Custom backend REST API

### WASM

- Browser localStorage as cache
- REST API for sync
- IndexedDB for larger saves

## References

- [Steam Cloud Documentation](https://partner.steamgames.com/doc/features/cloud)
