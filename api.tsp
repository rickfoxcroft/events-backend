import "@typespec/http";
import "@typespec/rest";
import "@typespec/openapi3";

using TypeSpec.Http;
using TypeSpec.Rest;

@service({
  title: "Event Venue Booking API",
})
namespace EventApp;

model VenueDTO {
  id: string;
  name: string;
  location: string;
  capacity: int32;
  owner_id: string;
}

model VenueInputDTO {
  name: string;
  location: string;
  capacity: int32;
}

model BookingDTO {
  id: string;
  venue_id: string;
  user_id: string;
  start_time: string;
  end_time: string;
}

@route("/venues")
interface Venues {
  @get list(): VenueDTO[];
  @post create(@body venue: VenueInputDTO): { @statusCode statusCode: 201 };
}
