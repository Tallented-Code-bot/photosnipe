# Testing Guide

## Overview

This document describes how to run tests for the PhotoSnipe backend API.

## Test Structure

Tests are organized into the following categories:

### 1. Unit Tests (No MongoDB Required)
- **Response Types** (`responses.rs`) - Tests for API response structures and error handling
- **Models** (future) - Tests for data structures

### 2. Integration Tests (MongoDB Required)
- **Person Endpoints** (`handlers/persons.rs`) - CRUD operations for persons
- **Snipe Endpoints** (`handlers/snipes.rs`) - CRUD operations for snipes
- **Statistics Endpoints** (`handlers/stats.rs`) - Statistics and leaderboards

## Prerequisites

### For Unit Tests
No special requirements. Simply run:
```bash
cargo test responses::tests
```

### For Integration Tests
Integration tests require a running MongoDB instance.

#### Option 1: Local MongoDB
Install and start MongoDB:
```bash
# Ubuntu/Debian
sudo apt-get install mongodb
sudo systemctl start mongod

# macOS
brew install mongodb-community
brew services start mongodb-community

# Verify it's running
systemctl is-active mongod  # Linux
# or
brew services list  # macOS
```

#### Option 2: Docker
Run MongoDB in a Docker container:
```bash
docker run -d -p 27017:27017 --name mongodb-test mongo:latest
```

#### Option 3: Skip Integration Tests
Run only unit tests:
```bash
cargo test --lib  # This won't work for our setup since tests are in bin
cargo test responses::tests  # Run specific unit tests
```

## Running Tests

### Run All Tests
```bash
cd backend
cargo test
```

### Run Specific Test Suites

**Response type tests (unit tests):**
```bash
cargo test responses::tests
```

**Person endpoint tests:**
```bash
cargo test handlers::persons::tests
```

**Snipe endpoint tests:**
```bash
cargo test handlers::snipes::tests
```

**Statistics endpoint tests:**
```bash
cargo test handlers::stats::tests
```

### Run Individual Tests
```bash
# Example: Run only the create person test
cargo test test_create_and_get_person -- --nocapture

# Example: Run only pagination validation test
cargo test test_pagination_params_validation
```

## Test Database

Integration tests use a separate test database: `photosnipe_test`

**Important Notes:**
- Tests automatically clean up after themselves
- Each test uses unique IDs to avoid conflicts
- The test database is separate from your production database
- You can manually clean the test database if needed:
  ```bash
  mongosh photosnipe_test --eval "db.dropDatabase()"
  ```

## Test Coverage

### Person Endpoints (8 tests)
- ✅ Create and get person
- ✅ List persons with pagination
- ✅ Get non-existent person (404)
- ✅ Update person
- ✅ Delete person
- ✅ Create duplicate person (409)
- ✅ Invalid pagination parameters

### Snipe Endpoints (10 tests)
- ✅ Create and get snipe
- ✅ List snipes
- ✅ Get snipes by channel
- ✅ Get snipes by guild
- ✅ Search snipes with filters
- ✅ Create snipe with missing person (validation)
- ✅ Update snipe
- ✅ Delete snipe
- ✅ Invalid ObjectId format

### Statistics Endpoints (3 tests)
- ✅ Get global statistics
- ✅ Get top snipers leaderboard
- ✅ Get top snipees leaderboard

### Response Types (11 tests)
- ✅ API response success
- ✅ API response with metadata
- ✅ Response meta has_more calculation
- ✅ API error types (not found, bad request, validation, conflict, internal)
- ✅ Pagination params validation
- ✅ Snipe search params

## Test Results Example

When MongoDB is running and all tests pass:

```
running 29 tests
test responses::tests::test_api_error_bad_request ... ok
test responses::tests::test_api_error_conflict ... ok
test responses::tests::test_api_error_internal ... ok
test responses::tests::test_api_error_not_found ... ok
test responses::tests::test_api_error_validation ... ok
test responses::tests::test_api_response_success ... ok
test responses::tests::test_api_response_with_meta ... ok
test responses::tests::test_pagination_params_validation ... ok
test responses::tests::test_response_meta_has_more ... ok
test responses::tests::test_snipe_search_params_pagination ... ok
test handlers::persons::tests::test_create_and_get_person ... ok
test handlers::persons::tests::test_create_duplicate_person ... ok
test handlers::persons::tests::test_delete_person ... ok
test handlers::persons::tests::test_get_nonexistent_person ... ok
test handlers::persons::tests::test_invalid_pagination ... ok
test handlers::persons::tests::test_list_persons_pagination ... ok
test handlers::persons::tests::test_update_person ... ok
test handlers::snipes::tests::test_create_and_get_snipe ... ok
test handlers::snipes::tests::test_create_snipe_missing_person ... ok
test handlers::snipes::tests::test_delete_snipe ... ok
test handlers::snipes::tests::test_get_snipes_by_channel ... ok
test handlers::snipes::tests::test_get_snipes_by_guild ... ok
test handlers::snipes::tests::test_invalid_object_id ... ok
test handlers::snipes::tests::test_list_snipes ... ok
test handlers::snipes::tests::test_search_snipes ... ok
test handlers::snipes::tests::test_update_snipe ... ok
test handlers::stats::tests::test_get_global_stats ... ok
test handlers::stats::tests::test_get_top_snipees ... ok
test handlers::stats::tests::test_get_top_snipers ... ok

test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Troubleshooting

### "Failed to connect to MongoDB"
**Problem:** Integration tests fail with connection errors.

**Solution:** 
1. Verify MongoDB is running: `systemctl status mongod` or `docker ps`
2. Check the connection string in tests (default: `mongodb://localhost:27017`)
3. Ensure MongoDB is accessible on port 27017

### "assertion failed: left == right"
**Problem:** Response status codes don't match expected values.

**Solution:**
1. Check the test output for error details
2. Verify MongoDB is running and accessible
3. Check that test data was properly set up

### Test Hangs or Times Out
**Problem:** Tests don't complete.

**Solution:**
1. Verify MongoDB is responsive
2. Check for port conflicts (27017)
3. Restart MongoDB service

## CI/CD Integration

For CI/CD pipelines, you'll need to:

1. Start MongoDB before running tests:
   ```yaml
   services:
     - mongodb
   
   before_script:
     - cargo build
   
   test:
     script:
       - cargo test
   ```

2. Or use a Docker Compose setup:
   ```yaml
   version: '3'
   services:
     mongodb:
       image: mongo:latest
       ports:
         - "27017:27017"
     
     tests:
       build: .
       depends_on:
         - mongodb
       command: cargo test
   ```

## Future Improvements

- [ ] Add mock MongoDB for tests (mongodb-memory-server equivalent)
- [ ] Add test fixtures for common data
- [ ] Add performance/load tests
- [ ] Add test coverage reporting
- [ ] Add integration tests for Discord bot
- [ ] Add end-to-end tests with frontend
