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
}

model ImageUploadURLResponseDTO {
  upload_url: string;
  @format("uuid")
  image_id: string;
}

model ImageUploadCompleteDTO {
  @format("uuid")
  image_id: string;
  filename: string;
}

@route("/venues")
interface Venues {
  @get list(): VenueDTO[];
  @post create(@body venue: VenueInputDTO): {
    @statusCode statusCode: 201;
  };
  
  @route("/{id}/images/upload-url")
  @post getUploadUrl(@path @format("uuid") id: string): ImageUploadURLResponseDTO;

  @route("/{id}/images/complete")
  @post completeUpload(@path @format("uuid") id: string, @body data: ImageUploadCompleteDTO): {
    @statusCode statusCode: 200;
  };
}
