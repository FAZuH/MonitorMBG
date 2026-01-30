# MonitorMBG API Schema Documentation

## Authentication Mechanism

The API uses **JWT (JSON Web Tokens)** for authentication.
- **Header**: `Authorization: Bearer <your_jwt_token>`
- **Token Expiration**: 1 hour
- **Token Refresh**: Use `/auth/refresh` endpoint before expiration

All endpoints marked with **[Auth Required]** expect a valid JWT in the header.

**Standard Error Response:**
```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable error message",
    "statusCode": 400,
    "details": {}
  }
}
```

## Base URL
`/api`

## Date/Time Format
All dates use **ISO 8601** format: `YYYY-MM-DDTHH:mm:ssZ` (e.g., `2025-11-25T10:30:00Z`)

## Pagination
All paginated endpoints use consistent parameters:
- `limit`: Number of items per page (default: 20, max: 100)
- `offset`: Number of items to skip (default: 0)

---

## 1. Authentication & User Management

### Login
**POST** `/auth/login`

**Access:** Public  
**Rate Limit:** 10 requests per IP per minute

Authenticates a user using their unique institution code.

**Request Body:**
```json
{
  "uniqueCode": "KTCH-1234-5678"
}
```
- `uniqueCode` (string, required): Unique institution identifier

**Success Response:** `200 OK`
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refreshToken": "refresh_token_string",
  "expiresIn": 3600,
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "personalName": "Budi Santoso",
    "institutionName": "Dapur Sehat Jakarta Pusat",
    "role": "kitchen",
    "uniqueCode": "KTCH-1234-5678",
    "phone": "08123456789",
    "verified": true,
    "institutionId": "inst_uuid",
    "createdAt": "2025-01-15T08:00:00Z",
    "lastLogin": "2025-01-30T14:23:45Z"
  }
}
```

**Error Responses:**
- `400 Bad Request`: Missing or invalid uniqueCode
- `401 Unauthorized`: Invalid credentials
- `429 Too Many Requests`: Rate limit exceeded

---

### Register
**POST** `/auth/register`

**Access:** Public  
**Rate Limit:** 5 requests per IP per hour

Registers a new account.

**Request Body:**
```json
{
  "uniqueCode": "KTCH-1234-5678",
  "institutionName": "Dapur Sehat Jakarta Pusat",
  "personalName": "Budi Santoso",
  "phone": "08123456789",
  "ktpPhoto": "base64_encoded_image_string",
  "consentGiven": true
}
```
- `uniqueCode` (string, required): Unique institution code
- `institutionName` (string, required): Institution name
- `personalName` (string, required): User's full name
- `phone` (string, required): Phone number in format 08XXXXXXXXX
- `ktpPhoto` (string, required): Base64 encoded KTP image (max 5MB, JPEG/PNG)
- `consentGiven` (boolean, required): Must be `true`

**Success Response:** `201 Created`
```json
{
  "success": true,
  "message": "Registration successful. Please verify your phone number.",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "personalName": "Budi Santoso",
    "institutionName": "Dapur Sehat Jakarta Pusat",
    "role": "kitchen",
    "uniqueCode": "KTCH-1234-5678",
    "phone": "08123456789",
    "verified": false,
    "institutionId": "inst_uuid",
    "createdAt": "2025-01-30T14:23:45Z"
  }
}
```

**Error Responses:**
- `400 Bad Request`: Missing required fields, invalid phone format, or image too large
- `409 Conflict`: uniqueCode or phone already registered
- `429 Too Many Requests`: Rate limit exceeded

---

### Send OTP
**POST** `/auth/otp/send`

**Access:** Public  
**Rate Limit:** 3 requests per phone number per hour

Sends a WhatsApp OTP for phone verification.

**Request Body:**
```json
{
  "phone": "08123456789"
}
```
- `phone` (string, required): Phone number in format 08XXXXXXXXX

**Success Response:** `200 OK`
```json
{
  "success": true,
  "message": "OTP sent via WhatsApp",
  "referenceId": "otp_550e8400-e29b-41d4-a716-446655440000",
  "expiresIn": 300
}
```

**Error Responses:**
- `400 Bad Request`: Invalid phone format
- `429 Too Many Requests`: Rate limit exceeded (3 per hour)
- `503 Service Unavailable`: WhatsApp service unavailable

---

### Verify OTP
**POST** `/auth/otp/verify`

**Access:** Public  
**Rate Limit:** 5 attempts per referenceId

Verifies the OTP code sent via WhatsApp.

**Request Body:**
```json
{
  "phone": "08123456789",
  "code": "123456",
  "referenceId": "otp_550e8400-e29b-41d4-a716-446655440000"
}
```
- `phone` (string, required): Phone number
- `code` (string, required): 6-digit OTP code
- `referenceId` (string, required): Reference ID from send request

**Success Response:** `200 OK`
```json
{
  "success": true,
  "message": "Phone verified successfully",
  "verified": true
}
```

**Error Responses:**
- `400 Bad Request`: Invalid OTP code or expired
- `404 Not Found`: Invalid referenceId
- `429 Too Many Requests`: Too many verification attempts

---

### Refresh Token
**POST** `/auth/refresh`

**Access:** Public  
**Rate Limit:** 20 requests per user per hour

Refreshes an expired or expiring JWT token.

**Request Body:**
```json
{
  "refreshToken": "refresh_token_string"
}
```

**Success Response:** `200 OK`
```json
{
  "token": "new_jwt_token_string",
  "refreshToken": "new_refresh_token_string",
  "expiresIn": 3600
}
```

**Error Responses:**
- `400 Bad Request`: Missing refreshToken
- `401 Unauthorized`: Invalid or expired refreshToken

---

### Get Current User
**GET** `/auth/me`

**Access:** Auth Required  
**Rate Limit:** 60 requests per user per minute

Retrieves the currently authenticated user's profile.

**Success Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "personalName": "Budi Santoso",
  "institutionName": "Dapur Sehat Jakarta Pusat",
  "role": "kitchen",
  "uniqueCode": "KTCH-1234-5678",
  "phone": "08123456789",
  "verified": true,
  "institutionId": "inst_uuid",
  "createdAt": "2025-01-15T08:00:00Z",
  "lastLogin": "2025-01-30T14:23:45Z"
}
```

**Error Responses:**
- `401 Unauthorized`: Invalid or missing token

---

## 2. Kitchen Management

### List Kitchens
**GET** `/kitchens`

**Access:** Public  
**Rate Limit:** 60 requests per IP per minute

Retrieves a paginated list of kitchens with optional filtering.

**Query Parameters:**
- `q` (string, optional): Search query for kitchen name
- `loc` (string, optional): Location filter (city/province)
- `type` (string, optional): Kitchen type filter
- `minRating` (number, optional): Minimum rating (0-5)
- `certified` (boolean, optional): Filter by certification status
- `limit` (number, optional): Items per page (default: 20, max: 100)
- `offset` (number, optional): Pagination offset (default: 0)
- `sort` (string, optional): Sort field (options: `name`, `rating`, `mealsServed`, default: `rating`)
- `order` (string, optional): Sort order (`asc` or `desc`, default: `desc`)

**Success Response:** `200 OK`
```json
{
  "data": [
    {
      "id": "kitchen_550e8400-e29b-41d4-a716-446655440000",
      "name": "Dapur Sehat Jakarta Pusat",
      "location": "Jakarta Pusat, DKI Jakarta",
      "type": "Central Kitchen",
      "mealsServed": 5000,
      "certifications": ["HACCP Certified", "Halal MUI"],
      "image": "https://storage.monitormbg.go.id/kitchens/img001.jpg",
      "rating": 4.8,
      "totalReviews": 156,
      "createdAt": "2024-06-15T08:00:00Z",
      "updatedAt": "2025-01-30T10:15:00Z"
    }
  ],
  "pagination": {
    "total": 1248,
    "limit": 20,
    "offset": 0,
    "hasMore": true
  }
}
```

**Error Responses:**
- `400 Bad Request`: Invalid query parameters

---

### Get Kitchen Detail
**GET** `/kitchens/:id`

**Access:** Public  
**Rate Limit:** 120 requests per IP per minute

Retrieves detailed information about a specific kitchen.

**Path Parameters:**
- `id` (string, required): Kitchen UUID

**Success Response:** `200 OK`
```json
{
  "id": "kitchen_550e8400-e29b-41d4-a716-446655440000",
  "name": "Dapur Sehat Jakarta Pusat",
  "location": "Jakarta Pusat, DKI Jakarta",
  "address": "Jl. Merdeka No. 123, Jakarta Pusat",
  "type": "Central Kitchen",
  "mealsServed": 5000,
  "certifications": ["HACCP Certified", "Halal MUI"],
  "image": "https://storage.monitormbg.go.id/kitchens/img001.jpg",
  "rating": 4.8,
  "totalReviews": 156,
  "contactPhone": "021-1234567",
  "contactEmail": "info@dapursehat.id",
  "operatingHours": "05:00-14:00",
  "capacity": 8000,
  "performanceBadges": [
    {
      "type": "gold",
      "title": "Best Practice",
      "description": "Excellent HACCP compliance for 6 consecutive months",
      "earnedDate": "2025-11-01T00:00:00Z",
      "icon": "https://storage.monitormbg.go.id/badges/gold.svg"
    }
  ],
  "complianceTrend": [
    {
      "month": "2025-11",
      "score": 95.2,
      "incidents": 0
    },
    {
      "month": "2025-10",
      "score": 93.8,
      "incidents": 1
    }
  ],
  "createdAt": "2024-06-15T08:00:00Z",
  "updatedAt": "2025-01-30T10:15:00Z"
}
```

**Error Responses:**
- `404 Not Found`: Kitchen not found

---

### Get Kitchen Stats
**GET** `/kitchens/:id/stats`

**Access:** Public  
**Rate Limit:** 60 requests per IP per minute

Retrieves aggregated statistics and HACCP scores for a kitchen.

**Path Parameters:**
- `id` (string, required): Kitchen UUID

**Success Response:** `200 OK`
```json
{
  "kitchenId": "kitchen_550e8400-e29b-41d4-a716-446655440000",
  "totalReviews": 156,
  "verifiedReviews": 142,
  "averageRating": 4.7,
  "haccpScores": {
    "taste": 4.5,
    "hygiene": 4.8,
    "freshness": 4.7,
    "temperature": 4.9,
    "packaging": 4.6,
    "handling": 4.8
  },
  "reviewDistribution": {
    "5": 98,
    "4": 42,
    "3": 12,
    "2": 3,
    "1": 1
  },
  "lastUpdated": "2025-01-30T14:00:00Z"
}
```

**Error Responses:**
- `404 Not Found`: Kitchen not found

---

### Get Multiple Kitchens
**GET** `/kitchens/batch`

**Access:** Public  
**Rate Limit:** 30 requests per IP per minute

Retrieves multiple kitchens by IDs in a single request.

**Query Parameters:**
- `ids` (string, required): Comma-separated kitchen UUIDs (max 50)

**Success Response:** `200 OK`
```json
{
  "data": [
    {
      "id": "kitchen_uuid_1",
      "name": "Kitchen 1",
      ...
    },
    {
      "id": "kitchen_uuid_2",
      "name": "Kitchen 2",
      ...
    }
  ],
  "notFound": ["kitchen_uuid_3"]
}
```

**Error Responses:**
- `400 Bad Request`: Missing ids or too many ids (>50)

---

## 3. Reviews & HACCP

### Submit Review
**POST** `/reviews`

**Access:** Auth Required  
**Rate Limit:** 10 reviews per user per day

Submits a new HACCP review for a kitchen.

**Request Body:**
```json
{
  "kitchenId": "kitchen_550e8400-e29b-41d4-a716-446655440000",
  "reviewerName": "SD Negeri 01 Jakarta",
  "reviewerType": "consumer",
  "ratings": {
    "taste": 4.5,
    "hygiene": 5.0,
    "freshness": 4.8,
    "temperature": 5.0,
    "packaging": 4.7,
    "handling": 5.0
  },
  "comment": "Food arrived in excellent condition with proper temperature control.",
  "photos": [
    "https://storage.monitormbg.go.id/reviews/photo1.jpg",
    "https://storage.monitormbg.go.id/reviews/photo2.jpg"
  ],
  "deliveryDate": "2025-01-30T11:00:00Z",
  "mealType": "lunch"
}
```
- `kitchenId` (string, required): UUID of the kitchen being reviewed
- `reviewerName` (string, required): Name of reviewing institution
- `reviewerType` (string, required): One of `consumer`, `supplier`, `kitchen`
- `ratings` (object, required): HACCP ratings object
  - All fields (number, required): Values 0.0-5.0, step 0.1
- `comment` (string, required): Review text (min 10 chars, max 1000 chars)
- `photos` (array, optional): Array of uploaded photo URLs (max 5, use `/upload/image` first)
- `deliveryDate` (string, optional): When meal was delivered
- `mealType` (string, optional): One of `breakfast`, `lunch`, `dinner`, `snack`

**Success Response:** `201 Created`
```json
{
  "id": "review_550e8400-e29b-41d4-a716-446655440000",
  "kitchenId": "kitchen_550e8400-e29b-41d4-a716-446655440000",
  "reviewerName": "SD Negeri 01 Jakarta",
  "reviewerType": "consumer",
  "ratings": {
    "taste": 4.5,
    "hygiene": 5.0,
    "freshness": 4.8,
    "temperature": 5.0,
    "packaging": 4.7,
    "handling": 5.0
  },
  "comment": "Food arrived in excellent condition with proper temperature control.",
  "photos": ["url1", "url2"],
  "deliveryDate": "2025-01-30T11:00:00Z",
  "mealType": "lunch",
  "verified": false,
  "verificationStatus": "pending",
  "reportSource": "public",
  "confidenceLevel": "medium",
  "createdAt": "2025-01-30T14:30:00Z",
  "updatedAt": "2025-01-30T14:30:00Z"
}
```

**Error Responses:**
- `400 Bad Request`: Invalid ratings, missing required fields, or invalid photo URLs
- `401 Unauthorized`: Missing or invalid token
- `404 Not Found`: Kitchen not found
- `429 Too Many Requests`: Daily review limit exceeded

---

### Get Kitchen Reviews
**GET** `/reviews/kitchen/:kitchenId`

**Access:** Public  
**Rate Limit:** 60 requests per IP per minute

Retrieves paginated reviews for a specific kitchen.

**Path Parameters:**
- `kitchenId` (string, required): Kitchen UUID

**Query Parameters:**
- `limit` (number, optional): Items per page (default: 20, max: 100)
- `offset` (number, optional): Pagination offset (default: 0)
- `verified` (boolean, optional): Filter by verification status
- `minRating` (number, optional): Minimum average rating (0-5)
- `reviewerType` (string, optional): Filter by reviewer type
- `sort` (string, optional): Sort by `date`, `rating` (default: `date`)
- `order` (string, optional): `asc` or `desc` (default: `desc`)

**Success Response:** `200 OK`
```json
{
  "data": [
    {
      "id": "review_550e8400-e29b-41d4-a716-446655440000",
      "kitchenId": "kitchen_550e8400-e29b-41d4-a716-446655440000",
      "reviewerName": "SD Negeri 01 Jakarta",
      "reviewerType": "consumer",
      "ratings": {
        "taste": 4.5,
        "hygiene": 5.0,
        "freshness": 4.8,
        "temperature": 5.0,
        "packaging": 4.7,
        "handling": 5.0
      },
      "averageRating": 4.8,
      "comment": "Food arrived in excellent condition.",
      "photos": ["url1", "url2"],
      "deliveryDate": "2025-01-30T11:00:00Z",
      "mealType": "lunch",
      "verified": true,
      "verificationStatus": "verified",
      "reportSource": "public",
      "confidenceLevel": "high",
      "rootCauses": [],
      "evidence": {
        "photoVerified": true,
        "locationVerified": true,
        "crossReferenced": true
      },
      "createdAt": "2025-01-30T14:30:00Z",
      "updatedAt": "2025-01-30T15:00:00Z"
    }
  ],
  "pagination": {
    "total": 156,
    "limit": 20,
    "offset": 0,
    "hasMore": true
  }
}
```

**Error Responses:**
- `400 Bad Request`: Invalid query parameters
- `404 Not Found`: Kitchen not found

---

### Get Public Reviews
**GET** `/reviews/public`

**Access:** Public  
**Rate Limit:** 60 requests per IP per minute

Retrieves verified reviews for the public dashboard/feed.

**Query Parameters:**
- `limit` (number, optional): Items per page (default: 20, max: 100)
- `offset` (number, optional): Pagination offset (default: 0)
- `province` (string, optional): Filter by province
- `minRating` (number, optional): Minimum rating filter

**Success Response:** `200 OK`
```json
{
  "data": [
    {
      "id": "review_550e8400-e29b-41d4-a716-446655440000",
      "kitchenId": "kitchen_550e8400-e29b-41d4-a716-446655440000",
      "kitchenName": "Dapur Sehat Jakarta",
      "location": "Jakarta Pusat",
      "province": "DKI Jakarta",
      "averageRating": 4.8,
      "reviewerType": "Sekolah",
      "comment": "Excellent food quality and hygiene practices.",
      "photo": "https://storage.monitormbg.go.id/reviews/photo1.jpg",
      "verified": true,
      "haccpScores": {
        "taste": 4.5,
        "hygiene": 5.0,
        "freshness": 4.8,
        "temperature": 5.0,
        "packaging": 4.7,
        "handling": 5.0
      },
      "createdAt": "2025-01-30T14:30:00Z"
    }
  ],
  "pagination": {
    "total": 8543,
    "limit": 20,
    "offset": 0,
    "hasMore": true
  }
}
```

**Error Responses:**
- `400 Bad Request`: Invalid query parameters

---

### Update Review
**PATCH** `/reviews/:id`

**Access:** Auth Required (Own reviews only)  
**Rate Limit:** 20 requests per user per hour

Updates an existing review. Only the review author can update.

**Path Parameters:**
- `id` (string, required): Review UUID

**Request Body:**
```json
{
  "ratings": {
    "taste": 4.5,
    "hygiene": 5.0,
    "freshness": 4.8,
    "temperature": 5.0,
    "packaging": 4.7,
    "handling": 5.0
  },
  "comment": "Updated review text",
  "photos": ["url1", "url2"]
}
```
- All fields are optional
- Can only update if review is not yet verified

**Success Response:** `200 OK`
```json
{
  "id": "review_uuid",
  "message": "Review updated successfully",
  ...
}
```

**Error Responses:**
- `400 Bad Request`: Invalid data or review already verified
- `401 Unauthorized`: Not the review author
- `404 Not Found`: Review not found

---

### Delete Review
**DELETE** `/reviews/:id`

**Access:** Auth Required (Own reviews only)  
**Rate Limit:** 10 requests per user per hour

Deletes a review. Only the review author can delete.

**Path Parameters:**
- `id` (string, required): Review UUID

**Success Response:** `200 OK`
```json
{
  "success": true,
  "message": "Review deleted successfully"
}
```

**Error Responses:**
- `401 Unauthorized`: Not the review author
- `403 Forbidden`: Cannot delete verified reviews
- `404 Not Found`: Review not found

---

### Submit Batch Reviews
**POST** `/reviews/batch`

**Access:** Auth Required  
**Rate Limit:** 3 requests per user per day

Submit multiple reviews in one request (useful for schools reviewing multiple kitchens).

**Request Body:**
```json
{
  "reviews": [
    {
      "kitchenId": "kitchen_uuid_1",
      "reviewerName": "SD Negeri 01",
      "reviewerType": "consumer",
      "ratings": { ... },
      "comment": "..."
    },
    {
      "kitchenId": "kitchen_uuid_2",
      ...
    }
  ]
}
```
- `reviews` (array, required): Array of review objects (max 20)

**Success Response:** `201 Created`
```json
{
  "success": true,
  "created": 18,
  "failed": 2,
  "results": [
    {
      "kitchenId": "kitchen_uuid_1",
      "status": "created",
      "reviewId": "review_uuid_1"
    },
    {
      "kitchenId": "kitchen_uuid_2",
      "status": "failed",
      "error": "Kitchen not found"
    }
  ]
}
```

**Error Responses:**
- `400 Bad Request`: Invalid batch size or malformed data
- `401 Unauthorized`: Missing or invalid token

---

## 4. Incidents & Map

### Get Incidents
**GET** `/incidents`

**Access:** Public  
**Rate Limit:** 60 requests per IP per minute

Retrieves all food safety incidents for map visualization.

**Query Parameters:**
- `status` (string, optional): Filter by status (`resolved`, `investigating`, `critical`)
- `province` (string, optional): Filter by province
- `dateFrom` (string, optional): Start date (ISO 8601)
- `dateTo` (string, optional): End date (ISO 8601)
- `minVictims` (number, optional): Minimum number of victims
- `limit` (number, optional): Items per page (default: 100, max: 500)
- `offset` (number, optional): Pagination offset

**Success Response:** `200 OK`
```json
{
  "data": [
    {
      "id": "incident_550e8400-e29b-41d4-a716-446655440000",
      "location": "Jakarta Timur",
      "address": "Kelurahan Cakung, Kecamatan Cakung",
      "province": "DKI Jakarta",
      "kabupaten": "Jakarta Timur",
      "date": "2025-11-15T08:00:00Z",
      "victims": 45,
      "deaths": 0,
      "hospitalized": 12,
      "cause": "Bacterial contamination (Salmonella)",
      "status": "resolved",
      "severity": "medium",
      "coordinates": {
        "lat": -6.1751,
        "lng": 106.9250
      },
      "relatedKitchenId": "kitchen_uuid",
      "source": "Dinas Kesehatan DKI Jakarta",
      "createdAt": "2025-11-15T10:00:00Z",
      "updatedAt": "2025-11-20T14:00:00Z",
      "resolvedAt": "2025-11-20T14:00:00Z"
    }
  ],
  "pagination": {
    "total": 23,
    "limit": 100,
    "offset": 0,
    "hasMore": false
  }
}
```

**Error Responses:**
- `400 Bad Request`: Invalid query parameters

---

### Get Incident Detail
**GET** `/incidents/:id`

**Access:** Public  
**Rate Limit:** 120 requests per IP per minute

Retrieves detailed information about a specific incident.

**Path Parameters:**
- `id` (string, required): Incident UUID

**Success Response:** `200 OK`
```json
{
  "id": "incident_550e8400-e29b-41d4-a716-446655440000",
  "location": "Jakarta Timur",
  "address": "Kelurahan Cakung, Kecamatan Cakung",
  "province": "DKI Jakarta",
  "kabupaten": "Jakarta Timur",
  "date": "2025-11-15T08:00:00Z",
  "victims": 45,
  "deaths": 0,
  "hospitalized": 12,
  "cause": "Bacterial contamination (Salmonella)",
  "status": "resolved",
  "severity": "medium",
  "coordinates": {
    "lat": -6.1751,
    "lng": 106.9250
  },
  "relatedKitchenId": "kitchen_uuid",
  "relatedKitchenName": "Dapur X",
  "description": "Food poisoning incident affecting 45 students from 3 schools. Investigation traced contamination to improper storage temperatures during transport.",
  "timeline": [
    {
      "date": "2025-11-15T08:00:00Z",
      "event": "First cases reported",
      "description": "Students reported symptoms after lunch"
    },
    {
      "date": "2025-11-15T14:00:00Z",
      "event": "Investigation started",
      "description": "Health department initiated investigation"
    },
    {
      "date": "2025-11-20T14:00:00Z",
      "event": "Case resolved",
      "description": "All patients recovered, kitchen passed re-inspection"
    }
  ],
  "affectedInstitutions": [
    "SD Negeri 01 Cakung",
    "SD Negeri 05 Cakung",
    "SMP Negeri 112 Jakarta"
  ],
  "laboratoryResults": {
    "pathogen": "Salmonella enteritidis",
    "testDate": "2025-11-17T00:00:00Z",
    "confirmedBy": "Balai Laboratorium Kesehatan DKI"
  },
  "correctiveActions": [
    "Kitchen suspended operations for 3 days",
    "Staff retraining on temperature control",
    "Equipment sanitization",
    "Enhanced monitoring implemented"
  ],
  "source": "Dinas Kesehatan DKI Jakarta",
  "sourceUrl": "https://dinkes.jakarta.go.id/laporan/inc-2025-11-15",
  "createdAt": "2025-11-15T10:00:00Z",
  "updatedAt": "2025-11-20T14:00:00Z",
  "resolvedAt": "2025-11-20T14:00:00Z"
}
```

**Error Responses:**
- `404 Not Found`: Incident not found

---

## 5. Open Data & Statistics

### National Stats
**GET** `/stats/national`

**Access:** Public  
**Rate Limit:** 60 requests per IP per minute

Retrieves national-level summary statistics.

**Query Parameters:**
- `year` (number, optional): Year for statistics (default: current year)
- `month` (number, optional): Month for statistics (1-12, optional)

**Success Response:** `200 OK`
```json
{
  "period": {
    "year": 2025,
    "month": 1
  },
  "totalKitchens": 1248,
  "activeKitchens": 1205,
  "certifiedKitchens": 892,
  "totalReviews": 15678,
  "verifiedReviews": 14234,
  "averageRating": 4.6,
  "averageComplianceScore": 91.3,
  "totalIncidents": 23,
  "activeIncidents": 5,
  "resolvedIncidents": 18,
  "criticalIncidents": 2,
  "totalVictims": 456,
  "totalDeaths": 0,
  "provinceStats": [
    {
      "province": "DKI Jakarta",
      "totalKitchens": 145,
      "avgRating": 4.7,
      "incidents": 3
    }
  ],
  "lastUpdated": "2025-01-30T14:00:00Z"
}
```

**Error Responses:**
- `400 Bad Request`: Invalid year/month parameters

---

### Regional Stats
**GET** `/stats/regional`

**Access:** Public  
**Rate Limit:** 60 requests per IP per minute

Retrieves aggregated statistics by region (province/kabupaten).

**Query Parameters:**
- `province` (string, optional): Province name
- `kabupaten` (string, optional): Kabupaten/city name
- `year` (number, optional): Year (default: current year)
- `month` (number, optional): Month (1-12, optional)

**Success Response:** `200 OK`
```json
{
  "region": {
    "province": "DKI Jakarta",
    "kabupaten": "Jakarta Pusat"
  },
  "period": {
    "year": 2025,
    "month": 1
  },
  "totalKitchens": 45,
  "activeKitchens": 43,
  "certifiedKitchens": 38,
  "averageRating": 4.7,
  "averageComplianceScore": 92.5,
  "totalReviews": 2341,
  "totalIncidents": 1,
  "resolvedIncidents": 1,
  "activeIncidents": 0,
  "topPerformingKitchens": [
    {
      "id": "kitchen_uuid",
      "name": "Dapur Sehat Jakarta Pusat",
      "rating": 4.9,
      "complianceScore": 98.5
    }
  ],
  "lastUpdated": "2025-01-30T14:00:00Z"
}
```

**Error Responses:**
- `400 Bad Request`: Invalid parameters
- `404 Not Found`: Region not found

---

### Compliance Trends
**GET** `/stats/trends/compliance`

**Access:** Public  
**Rate Limit:** 60 requests per IP per minute

Retrieves compliance trend data over time.

**Query Parameters:**
- `province` (string, optional): Province filter
- `kabupaten` (string, optional): Kabupaten filter
- `kitchenId` (string, optional): Specific kitchen
- `months` (number, optional): Number of months to look back (default: 12, max: 36)

**Success Response:** `200 OK`
```json
{
  "region": {
    "province": "DKI Jakarta",
    "kabupaten": null
  },
  "period": {
    "from": "2024-02-01T00:00:00Z",
    "to": "2025-01-31T23:59:59Z"
  },
  "data": [
    {
      "month": "2024-02",
      "averageScore": 88.5,
      "incidents": 5,
      "reviews": 1234,
      "averageRating": 4.5
    },
    {
      "month": "2024-03",
      "averageScore": 89.2,
      "incidents": 3,
      "reviews": 1289,
      "averageRating": 4.6
    },
    {
      "month": "2025-01",
      "averageScore": 92.5,
      "incidents": 1,
      "reviews": 1456,
      "averageRating": 4.7
    }
  ],
  "summary": {
    "trend": "improving",
    "changePercent": 4.5,
    "totalIncidents": 23,
    "totalReviews": 15678
  },
  "lastUpdated": "2025-01-30T14:00:00Z"
}
```

**Error Responses:**
- `400 Bad Request`: Invalid parameters

---

### Incident Trends
**GET** `/stats/trends/incidents`

**Access:** Public  
**Rate Limit:** 60 requests per IP per minute

Retrieves incident trend data over time.

**Query Parameters:**
- `province` (string, optional): Province filter
- `months` (number, optional): Number of months (default: 12, max: 36)
- `groupBy` (string, optional): Group by `month`, `province`, `cause` (default: `month`)

**Success Response:** `200 OK`
```json
{
  "period": {
    "from": "2024-02-01T00:00:00Z",
    "to": "2025-01-31T23:59:59Z"
  },
  "groupBy": "month",
  "data": [
    {
      "month": "2024-02",
      "totalIncidents": 5,
      "totalVictims": 125,
      "deaths": 0,
      "topCause": "Bacterial contamination"
    },
    {
      "month": "2025-01",
      "totalIncidents": 1,
      "totalVictims": 23,
      "deaths": 0,
      "topCause": "Improper storage"
    }
  ],
  "summary": {
    "totalIncidents": 23,
    "totalVictims": 456,
    "totalDeaths": 0,
    "mostCommonCause": "Bacterial contamination",
    "trend": "decreasing"
  },
  "lastUpdated": "2025-01-30T14:00:00Z"
}
```

**Error Responses:**
- `400 Bad Request`: Invalid parameters

---

## 6. Utilities

### Upload Image
**POST** `/upload/image`

**Access:** Auth Required  
**Rate Limit:** 20 uploads per user per hour

Uploads an image file to cloud storage.

**Request:** `multipart/form-data`
- `file` (file, required): Image file
  - **Max size**: 5MB
  - **Allowed formats**: JPEG, PNG, WebP
  - **Recommended dimensions**: Max 2048x2048px

**Success Response:** `201 Created`
```json
{
  "success": true,
  "url": "https://storage.monitormbg.go.id/uploads/2025/01/30/550e8400-e29b-41d4-a716-446655440000.jpg",
  "fileName": "image.jpg",
  "size": 1048576,
  "mimeType": "image/jpeg",
  "width": 1920,
  "height": 1080,
  "uploadedAt": "2025-01-30T14:30:00Z"
}
```

**Error Responses:**
- `400 Bad Request`: File too large, invalid format, or missing file
- `401 Unauthorized`: Missing or invalid token
- `413 Payload Too Large`: File exceeds 5MB
- `429 Too Many Requests`: Rate limit exceeded
- `503 Service Unavailable`: Storage service error

---

### Upload Multiple Images
**POST** `/upload/images`

**Access:** Auth Required  
**Rate Limit:** 10 uploads per user per hour

Uploads multiple images in one request.

**Request:** `multipart/form-data`
- `files[]` (array of files, required): Image files (max 5 files)

**Success Response:** `201 Created`
```json
{
  "success": true,
  "uploaded": 3,
  "failed": 0,
  "results": [
    {
      "url": "https://storage.monitormbg.go.id/uploads/img1.jpg",
      "fileName": "image1.jpg",
      "size": 1048576
    },
    {
      "url": "https://storage.monitormbg.go.id/uploads/img2.jpg",
      "fileName": "image2.jpg",
      "size": 892341
    }
  ]
}
```

**Error Responses:**
- Same as single upload endpoint

---

### Health Check
**GET** `/health`

**Access:** Public  
**Rate Limit:** None

Returns API health status.

**Success Response:** `200 OK`
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "timestamp": "2025-01-30T14:30:00Z",
  "services": {
    "database": "healthy",
    "storage": "healthy",
    "whatsapp": "healthy"
  }
}
```

---

### Storage Backend Configuration

The API supports multiple storage backends for file uploads. The backend is configured via environment variables and is transparent to API clients.

**Supported Backends:**

| Backend | Type | Use Case |
|---------|------|----------|
| Local Filesystem | `local` | Development, single-server deployments |
| Amazon S3 | `s3` | Production, scalable storage |
| S3-Compatible | `s3` | MinIO, DigitalOcean Spaces, etc. |

**File Organization:**

Files are automatically organized by upload date:
```
uploads/
├── 2025/
│   ├── 01/
│   │   ├── 30/
│   │   │   ├── 550e8400-e29b-41d4-a716-446655440000.jpg
│   │   │   └── ...
│   │   └── 31/
│   └── 02/
└── ...
```

**File URL Format:**

- **Local**: `http://localhost:3000/uploads/2025/01/30/{file_id}`
- **S3**: `https://{bucket}.s3.{region}.amazonaws.com/uploads/2025/01/30/{file_id}`

**Storage Health Status:**

The health check endpoint reports storage status:
- `healthy`: Storage backend is accessible and operational
- `unhealthy`: Storage backend is inaccessible or experiencing issues

---

## TypeScript Type Definitions

```typescript
// ============================================================================
// User Types
// ============================================================================

type UserRole = 'kitchen' | 'supplier' | 'school';

interface User {
  id: string;
  personalName: string;
  institutionName: string;
  role: UserRole;
  uniqueCode: string;
  phone: string;
  verified: boolean;
  institutionId: string;
  createdAt: string;
  lastLogin?: string;
}

interface AuthResponse {
  token: string;
  refreshToken: string;
  expiresIn: number;
  user: User;
}

// ============================================================================
// Kitchen Types
// ============================================================================

interface Kitchen {
  id: string;
  name: string;
  location: string;
  address: string;
  type: string;
  mealsServed: number;
  certifications: string[];
  image: string;
  rating: number;
  totalReviews: number;
  contactPhone?: string;
  contactEmail?: string;
  operatingHours?: string;
  capacity?: number;
  performanceBadges?: PerformanceBadge[];
  complianceTrend?: ComplianceTrend[];
  createdAt: string;
  updatedAt: string;
}

interface PerformanceBadge {
  type: 'gold' | 'silver' | 'bronze';
  title: string;
  description: string;
  earnedDate: string;
  icon?: string;
}

interface ComplianceTrend {
  month: string;
  score: number;
  incidents: number;
}

interface KitchenStats {
  kitchenId: string;
  totalReviews: number;
  verifiedReviews: number;
  averageRating: number;
  haccpScores: HACCPRating;
  reviewDistribution: {
    5: number;
    4: number;
    3: number;
    2: number;
    1: number;
  };
  lastUpdated: string;
}

// ============================================================================
// Review Types
// ============================================================================

type ReviewerType = 'consumer' | 'supplier' | 'kitchen';
type VerificationStatus = 'pending' | 'in_progress' | 'verified' | 'rejected';
type ReportSource = 'public' | 'official_inspector' | 'health_worker';
type ConfidenceLevel = 'low' | 'medium' | 'high';
type MealType = 'breakfast' | 'lunch' | 'dinner' | 'snack';

interface HACCPRating {
  taste: number;        // 0.0 - 5.0, step 0.1
  hygiene: number;      // 0.0 - 5.0, step 0.1
  freshness: number;    // 0.0 - 5.0, step 0.1
  temperature: number;  // 0.0 - 5.0, step 0.1
  packaging: number;    // 0.0 - 5.0, step 0.1
  handling: number;     // 0.0 - 5.0, step 0.1
}

interface ReviewEvidence {
  photoVerified: boolean;
  locationVerified: boolean;
  crossReferenced: boolean;
}

interface Review {
  id: string;
  kitchenId: string;
  reviewerName: string;
  reviewerType: ReviewerType;
  ratings: HACCPRating;
  averageRating: number;
  comment: string;
  photos?: string[];
  deliveryDate?: string;
  mealType?: MealType;
  verified: boolean;
  verificationStatus: VerificationStatus;
  reportSource: ReportSource;
  confidenceLevel: ConfidenceLevel;
  rootCauses?: string[];
  evidence?: ReviewEvidence;
  createdAt: string;
  updatedAt: string;
}

// ============================================================================
// Incident Types
// ============================================================================

type IncidentStatus = 'resolved' | 'investigating' | 'critical';
type IncidentSeverity = 'low' | 'medium' | 'high' | 'critical';

interface Coordinates {
  lat: number;
  lng: number;
}

interface IncidentTimeline {
  date: string;
  event: string;
  description: string;
}

interface LaboratoryResults {
  pathogen: string;
  testDate: string;
  confirmedBy: string;
}

interface Incident {
  id: string;
  location: string;
  address: string;
  province: string;
  kabupaten: string;
  date: string;
  victims: number;
  deaths: number;
  hospitalized: number;
  cause: string;
  status: IncidentStatus;
  severity: IncidentSeverity;
  coordinates: Coordinates;
  relatedKitchenId?: string;
  relatedKitchenName?: string;
  description?: string;
  timeline?: IncidentTimeline[];
  affectedInstitutions?: string[];
  laboratoryResults?: LaboratoryResults;
  correctiveActions?: string[];
  source: string;
  sourceUrl?: string;
  createdAt: string;
  updatedAt: string;
  resolvedAt?: string;
}

// ============================================================================
// Statistics Types
// ============================================================================

interface NationalStats {
  period: {
    year: number;
    month?: number;
  };
  totalKitchens: number;
  activeKitchens: number;
  certifiedKitchens: number;
  totalReviews: number;
  verifiedReviews: number;
  averageRating: number;
  averageComplianceScore: number;
  totalIncidents: number;
  activeIncidents: number;
  resolvedIncidents: number;
  criticalIncidents: number;
  totalVictims: number;
  totalDeaths: number;
  provinceStats: ProvinceStats[];
  lastUpdated: string;
}

interface ProvinceStats {
  province: string;
  totalKitchens: number;
  avgRating: number;
  incidents: number;
}

interface RegionalStats {
  region: {
    province: string;
    kabupaten?: string;
  };
  period: {
    year: number;
    month?: number;
  };
  totalKitchens: number;
  activeKitchens: number;
  certifiedKitchens: number;
  averageRating: number;
  averageComplianceScore: number;
  totalReviews: number;
  totalIncidents: number;
  resolvedIncidents: number;
  activeIncidents: number;
  topPerformingKitchens: TopKitchen[];
  lastUpdated: string;
}

interface TopKitchen {
  id: string;
  name: string;
  rating: number;
  complianceScore: number;
}

interface ComplianceTrendData {
  month: string;
  averageScore: number;
  incidents: number;
  reviews: number;
  averageRating: number;
}

interface ComplianceTrends {
  region: {
    province?: string;
    kabupaten?: string;
  };
  period: {
    from: string;
    to: string;
  };
  data: ComplianceTrendData[];
  summary: {
    trend: 'improving' | 'stable' | 'declining';
    changePercent: number;
    totalIncidents: number;
    totalReviews: number;
  };
  lastUpdated: string;
}

// ============================================================================
// Pagination Types
// ============================================================================

interface Pagination {
  total: number;
  limit: number;
  offset: number;
  hasMore: boolean;
}

interface PaginatedResponse<T> {
  data: T[];
  pagination: Pagination;
}

// ============================================================================
// Error Types
// ============================================================================

interface APIError {
  error: {
    code: string;
    message: string;
    statusCode: number;
    details?: Record<string, any>;
  };
}

// ============================================================================
// Upload Types
// ============================================================================

interface ImageUploadResponse {
  success: true;
  url: string;
  fileName: string;
  size: number;
  mimeType: string;
  width: number;
  height: number;
  uploadedAt: string;
}

interface MultiImageUploadResponse {
  success: true;
  uploaded: number;
  failed: number;
  results: {
    url: string;
    fileName: string;
    size: number;
  }[];
}
```

---

## Rate Limiting

All endpoints are rate limited. Rate limit headers are included in responses:

```
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 58
X-RateLimit-Reset: 1706634000
```

When rate limit is exceeded, API returns `429 Too Many Requests`:

```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Too many requests. Please try again in 45 seconds.",
    "statusCode": 429,
    "details": {
      "retryAfter": 45
    }
  }
}
```

---

## Common Error Codes

| Code | Status | Description |
|------|--------|-------------|
| `INVALID_TOKEN` | 401 | JWT token is invalid or expired |
| `MISSING_TOKEN` | 401 | Authorization header missing |
| `INVALID_CREDENTIALS` | 401 | Wrong uniqueCode or password |
| `RESOURCE_NOT_FOUND` | 404 | Requested resource doesn't exist |
| `VALIDATION_ERROR` | 400 | Request validation failed |
| `RATE_LIMIT_EXCEEDED` | 429 | Too many requests |
| `UNAUTHORIZED_ACCESS` | 403 | User lacks permission |
| `DUPLICATE_ENTRY` | 409 | Resource already exists |
| `SERVICE_UNAVAILABLE` | 503 | External service unavailable |
| `INTERNAL_ERROR` | 500 | Server error |

---

## Access Control Summary

| Endpoint | Access | Notes |
|----------|--------|-------|
| POST /auth/login | Public | Rate limited |
| POST /auth/register | Public | Rate limited |
| POST /auth/otp/send | Public | Rate limited per phone |
| POST /auth/otp/verify | Public | Limited attempts |
| POST /auth/refresh | Public | Rate limited |
| GET /auth/me | Auth Required | Own profile only |
| GET /kitchens | Public | - |
| GET /kitchens/:id | Public | - |
| GET /kitchens/:id/stats | Public | - |
| GET /kitchens/batch | Public | Max 50 IDs |
| POST /reviews | Auth Required | Daily limit |
| GET /reviews/kitchen/:id | Public | - |
| GET /reviews/public | Public | Verified only |
| PATCH /reviews/:id | Auth Required | Own reviews only |
| DELETE /reviews/:id | Auth Required | Own reviews, unverified only |
| POST /reviews/batch | Auth Required | Max 20 reviews |
| GET /incidents | Public | - |
| GET /incidents/:id | Public | - |
| GET /stats/* | Public | - |
| POST /upload/image | Auth Required | Rate limited |
| POST /upload/images | Auth Required | Max 5 files |

---

## Privacy & Data Visibility

**Public fields** (visible to all):
- Kitchen: All fields except internal contact details
- Review: All fields except reviewer personal identifiers (phone, KTP)
- Incident: All fields
- User: Only institutionName, role (personalName and phone are private)

**Private fields** (own user only):
- User phone number
- User personal name
- KTP photo
- Unverified reviews (before moderation)

**Admin-only fields**:
- User verification documents
- Review moderation data
- Incident investigation notes
