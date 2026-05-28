import "@typespec/http";
import "@typespec/rest";
import "@typespec/openapi3";

using TypeSpec.Http;
using TypeSpec.Rest;

@service(#{ title: "Event Venue Booking API" })
namespace EventApp;

model VenueDTO {
  @format("uuid")
  id: string;
  name: string;
  location: string;
  capacity: int32;
  price_per_hour: int32;
  @format("uuid")
  owner_id: string;
  images: VenueImageDTO[];
}

model VenueImageDTO {
  @format("uuid")
  id: string;
  url: string;
}

model VenueInputDTO {
  name: string;
  location: string;
  capacity: int32;
  price_per_hour: int32;
  image_ids: string[];
}

model ImageUploadURLResponseDTO {
  upload_url: string;
  @format("uuid")
  image_id: string;
}

model BookingDTO {
  @format("uuid")
  id: string;
  @format("uuid")
  venue_id: string;
  @format("uuid")
  user_id: string;
  start_time: string;
  end_time: string;
}

@route("/venues")
interface Venues {
  @get list(): VenueDTO[];
  @get get(@path id: string): VenueDTO;
  @post create(@body venue: VenueInputDTO): {
    @statusCode statusCode: 201;
  };
}

@route("/images")
interface Images {
  @route("/upload-url")
  @post getUploadUrl(): ImageUploadURLResponseDTO;
}
