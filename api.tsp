import "@typespec/http";
import "@typespec/rest";
import "@typespec/openapi3";

using TypeSpec.Http;
using TypeSpec.Rest;

@service(#{ title: "Event Venue Booking API" })
namespace EventApp;

model VenueDTO {
  id: string;
  name: string;
  location: string;
  capacity: int32;
  owner_id: string;
  images: VenueImageDTO[];
}

model VenueImageDTO {
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
  image_id: string;
}

model ImageUploadCompleteDTO {
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
  @post getUploadUrl(@path id: string): ImageUploadURLResponseDTO;

  @route("/{id}/images/complete")
  @post completeUpload(@path id: string, @body data: ImageUploadCompleteDTO): {
    @statusCode statusCode: 200;
  };
}
