const fs = require("fs");
const path = require("path");

async function seed() {
  // Clear existing data (optional but recommended for a clean seed)
  // Since we don't have a DELETE /venues endpoint yet, we can't easily do it via API.
  // However, for local dev, we can continue to use wrangler for the reset.

  const mockDataPath = path.join(
    __dirname,
    "../packages/web/src/data/mockVenues.ts",
  );
  const mockDataContent = fs.readFileSync(mockDataPath, "utf8");

  // Simple regex to extract venue objects from the TS file
  const venueRegex =
    /\{[\s\S]*?id: '(.*?)',[\s\S]*?name: '(.*?)',[\s\S]*?location: '(.*?)',[\s\S]*?capacity: (.*?),[\s\S]*?imageUrl:\s*'(.*?)',[\s\S]*?pricePerHour: (.*?),[\s\S]*?\}/g;

  let venues = [];
  let match;
  while ((match = venueRegex.exec(mockDataContent)) !== null) {
    venues.push({
      id: match[1],
      name: match[2],
      location: match[3],
      capacity: parseInt(match[4]),
      imageUrl: match[5],
      pricePerHour: parseInt(match[6]),
    });
  }

  const API_URL = "http://localhost:8787";

  console.log(`Found ${venues.length} venues to seed.`);

  for (const venue of venues) {
    console.log(`Seeding venue: ${venue.name}...`);

    // 1. Get upload URL
    const uploadUrlResp = await fetch(`${API_URL}/images/upload-url`, {
      method: "POST",
    });
    if (!uploadUrlResp.ok) {
      throw new Error(
        `Failed to get upload URL: ${uploadUrlResp.statusText}. Ensure your CF_IMAGES_API_TOKEN is set in .dev.vars`,
      );
    }
    const { upload_url, image_id } = await uploadUrlResp.json();

    // 2. Perform real upload to Cloudflare
    try {
      console.log(`  Downloading image for ${venue.name}...`);
      const imgResp = await fetch(venue.imageUrl);
      if (!imgResp.ok) {
        throw new Error(
          `Failed to download image from Unsplash: ${imgResp.statusText}`,
        );
      }
      const arrayBuffer = await imgResp.arrayBuffer();
      const buffer = Buffer.from(arrayBuffer);

      const formData = new FormData();
      formData.append(
        "file",
        new Blob([buffer], { type: "image/jpeg" }),
        "venue.jpg",
      );

      console.log(`  Uploading to Cloudflare...`);
      const cfResp = await fetch(upload_url, {
        method: "POST",
        body: formData,
      });

      if (!cfResp.ok) {
        const text = await cfResp.text();
        throw new Error(
          `Failed to upload real image to Cloudflare: ${cfResp.statusText}. Body: ${text}`,
        );
      }
      console.log(`  Successfully uploaded image for ${venue.name}`);
    } catch (e) {
      console.error(
        `  Warning: Error during image processing for ${venue.name}: ${e.message}`,
      );
      console.log(`  Continuing with venue creation without image...`);
    }

    const payload = {
      name: venue.name,
      location: venue.location,
      capacity: venue.capacity,
      price_per_hour: venue.pricePerHour,
      image_ids: [image_id],
    };

    const createResp = await fetch(`${API_URL}/venues`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });

    if (createResp.ok) {
      console.log(`Successfully seeded ${venue.name}`);
    } else {
      console.error(`Failed to seed ${venue.name}: ${createResp.statusText}`);
      const text = await createResp.text();
      console.error(text);
    }
  }
}

seed().catch(console.error);
