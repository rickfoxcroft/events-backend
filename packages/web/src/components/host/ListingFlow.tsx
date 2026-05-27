import type { JSX } from 'preact';
import { useState } from 'preact/hooks';
import { api } from '../../lib/api';
import { schemas } from '../../types/api-zod';

interface VenueFormData {
  name: string;
  location: string;
  capacity: number;
  price_per_hour: number;
  files: File[];
}

export default function ListingFlow() {
  const [step, setStep] = useState(1);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [formData, setFormData] = useState<VenueFormData>({
    name: '',
    location: '',
    capacity: 0,
    price_per_hour: 0,
    files: [],
  });

  const nextStep = () => setStep((s) => s + 1);
  const prevStep = () => setStep((s) => s - 1);

  const handleInputChange = (e: JSX.TargetedEvent<HTMLInputElement, Event>) => {
    const { name, value } = e.currentTarget;
    setFormData((prev) => ({
      ...prev,
      [name]:
        name === 'capacity' || name === 'price_per_hour'
          ? parseInt(value) || 0
          : value,
    }));
  };

  const handleFileChange = (e: JSX.TargetedEvent<HTMLInputElement, Event>) => {
    const selectedFiles = Array.from(e.currentTarget.files || []);
    setFormData((prev) => ({
      ...prev,
      files: [...prev.files, ...selectedFiles],
    }));
  };

  const removeFile = (index: number) => {
    setFormData((prev) => ({
      ...prev,
      files: prev.files.filter((_, i) => i !== index),
    }));
  };

  const handleSubmit = async () => {
    setLoading(true);
    setError(null);
    try {
      // 1. Upload Images to Cloudflare
      const imageIds: string[] = [];

      for (const file of formData.files) {
        // Get upload URL (Type-safe request)
        const { data, error, response } = await api.POST('/images/upload-url');
        if (!response.ok || error || !data)
          throw new Error('Failed to get upload URL');

        const { upload_url, image_id } =
          schemas.ImageUploadURLResponseDTO.parse(data);

        // Upload File (External URL, so we still use native fetch)
        const uploadData = new FormData();
        uploadData.append('file', file);
        const cfResp = await fetch(upload_url, {
          method: 'POST',
          body: uploadData,
        });
        if (!cfResp.ok) throw new Error(`Failed to upload image: ${file.name}`);

        imageIds.push(image_id);
      }

      // 2. Create Venue (Type-safe request with body validation)
      const payload = {
        name: formData.name,
        location: formData.location,
        capacity: formData.capacity,
        price_per_hour: formData.price_per_hour,
        image_ids: imageIds,
      };

      const { error, response } = await api.POST('/venues', {
        body: payload,
      });

      if (!response.ok || error) throw new Error('Failed to create venue');

      setStep(4); // Success Step
    } catch (err) {
      const message =
        err instanceof Error ? err.message : 'An unexpected error occurred';
      setError(message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div class="shadow-primary/5 mx-auto max-w-2xl rounded-3xl border border-gray-100 bg-white p-8 shadow-xl md:p-12">
      {/* Progress Indicator */}
      {step < 4 && (
        <div class="mb-12 flex gap-2">
          {[1, 2, 3].map((i) => (
            <div
              key={i}
              class={`h-1.5 flex-1 rounded-full transition-colors duration-500 ${i <= step ? 'bg-primary' : 'bg-gray-100'}`}
            ></div>
          ))}
        </div>
      )}

      {/* Step 1: Basics */}
      {step === 1 && (
        <div class="animate-in fade-in slide-in-from-bottom-4 duration-500">
          <h2 class="text-navy mb-2 text-3xl font-bold tracking-tight">
            The Basics
          </h2>
          <p class="mb-8 text-lg text-gray-500">Tell us about your space.</p>

          <div class="space-y-6">
            <div>
              <label class="text-navy mb-2 ml-1 block text-sm font-bold tracking-widest uppercase">
                Venue Name
              </label>
              <input
                type="text"
                name="name"
                value={formData.name}
                onInput={handleInputChange}
                placeholder="e.g. The Indigo Loft"
                class="focus:border-primary focus:ring-primary w-full rounded-2xl border border-gray-200 p-4 text-lg transition-all outline-none focus:ring-1"
              />
            </div>
            <div>
              <label class="text-navy mb-2 ml-1 block text-sm font-bold tracking-widest uppercase">
                Location
              </label>
              <input
                type="text"
                name="location"
                value={formData.location}
                onInput={handleInputChange}
                placeholder="City, Neighborhood"
                class="focus:border-primary focus:ring-primary w-full rounded-2xl border border-gray-200 p-4 text-lg transition-all outline-none focus:ring-1"
              />
            </div>
            <div>
              <label class="text-navy mb-2 ml-1 block text-sm font-bold tracking-widest uppercase">
                Capacity
              </label>
              <input
                type="number"
                name="capacity"
                value={formData.capacity || ''}
                onInput={handleInputChange}
                placeholder="Max number of guests"
                class="focus:border-primary focus:ring-primary w-full rounded-2xl border border-gray-200 p-4 text-lg transition-all outline-none focus:ring-1"
              />
            </div>
          </div>

          <button
            disabled={
              !formData.name || !formData.location || !formData.capacity
            }
            onClick={nextStep}
            class="bg-primary hover:bg-navy hover:shadow-primary/20 mt-12 w-full rounded-full py-4 text-lg font-bold text-white shadow-lg transition-all disabled:cursor-not-allowed disabled:opacity-50"
          >
            Continue
          </button>
        </div>
      )}

      {/* Step 2: Pricing */}
      {step === 2 && (
        <div class="animate-in fade-in slide-in-from-bottom-4 duration-500">
          <h2 class="text-navy mb-2 text-3xl font-bold tracking-tight">
            Set Your Price
          </h2>
          <p class="mb-8 text-lg text-gray-500">
            How much do you want to charge per hour?
          </p>

          <div class="relative">
            <span class="absolute top-1/2 left-6 -translate-y-1/2 text-2xl font-bold text-gray-400">
              $
            </span>
            <input
              type="number"
              name="price_per_hour"
              value={formData.price_per_hour || ''}
              onInput={handleInputChange}
              placeholder="0"
              class="text-navy focus:border-primary focus:ring-primary w-full rounded-2xl border border-gray-200 p-4 pl-12 text-5xl font-bold transition-all outline-none focus:ring-1"
            />
            <span class="absolute top-1/2 right-6 -translate-y-1/2 text-lg font-bold tracking-widest text-gray-400 uppercase">
              per hour
            </span>
          </div>

          <div class="mt-12 flex gap-4">
            <button
              onClick={prevStep}
              class="text-navy flex-1 rounded-full border-2 border-gray-100 py-4 text-lg font-bold transition-all hover:border-gray-200"
            >
              Back
            </button>
            <button
              disabled={!formData.price_per_hour}
              onClick={nextStep}
              class="bg-primary hover:bg-navy hover:shadow-primary/20 flex-[2] rounded-full py-4 text-lg font-bold text-white shadow-lg transition-all"
            >
              Continue
            </button>
          </div>
        </div>
      )}

      {/* Step 3: Photos */}
      {step === 3 && (
        <div class="animate-in fade-in slide-in-from-bottom-4 duration-500">
          <h2 class="text-navy mb-2 text-3xl font-bold tracking-tight">
            Add Some Photos
          </h2>
          <p class="mb-8 text-lg text-gray-500">
            Show off your space to potential guests.
          </p>

          <div class="space-y-6">
            <div class="hover:border-primary/30 group relative flex cursor-pointer flex-col items-center justify-center rounded-3xl border-2 border-dashed border-gray-200 p-12 text-center transition-all">
              <input
                type="file"
                multiple
                accept="image/*"
                onChange={handleFileChange}
                class="absolute inset-0 cursor-pointer opacity-0"
              />
              <div class="group-hover:bg-primary/5 mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-gray-50 transition-colors">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="32"
                  height="32"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  class="group-hover:text-primary text-gray-400 transition-colors"
                >
                  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
                  <circle cx="9" cy="9" r="2" />
                  <path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" />
                </svg>
              </div>
              <span class="text-navy font-bold">
                Click to upload or drag and drop
              </span>
              <span class="mt-1 text-sm text-gray-400">
                PNG, JPG or WEBP (max 10MB)
              </span>
            </div>

            {formData.files.length > 0 && (
              <div class="mt-8 grid grid-cols-2 gap-4">
                {formData.files.map((file, index) => (
                  <div
                    key={index}
                    class="group relative aspect-square overflow-hidden rounded-2xl border border-gray-100"
                  >
                    <img
                      src={URL.createObjectURL(file)}
                      class="h-full w-full object-cover"
                    />
                    <button
                      onClick={() => removeFile(index)}
                      class="bg-navy/80 absolute top-2 right-2 flex h-8 w-8 items-center justify-center rounded-full text-white opacity-0 backdrop-blur-md transition-opacity group-hover:opacity-100"
                    >
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                      >
                        <path d="M18 6 6 18" />
                        <path d="m6 6 12 12" />
                      </svg>
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>

          {error && (
            <div class="mt-6 flex items-center gap-3 rounded-2xl border border-red-100 bg-red-50 p-4 text-sm font-medium text-red-600">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="18"
                height="18"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <circle cx="12" cy="12" r="10" />
                <line x1="12" x2="12" y1="8" y2="12" />
                <line x1="12" x2="12.01" y1="16" y2="16" />
              </svg>
              {error}
            </div>
          )}

          <div class="mt-12 flex gap-4">
            <button
              disabled={loading}
              onClick={prevStep}
              class="text-navy flex-1 rounded-full border-2 border-gray-100 py-4 text-lg font-bold transition-all hover:border-gray-200 disabled:opacity-50"
            >
              Back
            </button>
            <button
              disabled={loading || formData.files.length === 0}
              onClick={handleSubmit}
              class="bg-primary hover:bg-navy hover:shadow-primary/20 flex flex-[2] items-center justify-center gap-3 rounded-full py-4 text-lg font-bold text-white shadow-lg transition-all disabled:opacity-50"
            >
              {loading && (
                <span class="h-5 w-5 animate-spin rounded-full border-3 border-white/30 border-t-white"></span>
              )}
              {loading ? 'Publishing...' : 'List Your Venue'}
            </button>
          </div>
        </div>
      )}

      {/* Step 4: Success */}
      {step === 4 && (
        <div class="animate-in zoom-in-95 fade-in py-12 text-center duration-700">
          <div class="mx-auto mb-8 flex h-24 w-24 animate-bounce items-center justify-center rounded-full bg-green-50">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="48"
              height="48"
              viewBox="0 0 24 24"
              fill="none"
              stroke="rgb(34, 197, 94)"
              stroke-width="3"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path d="M20 6 9 17l-5-5" />
            </svg>
          </div>
          <h2 class="text-navy mb-4 text-4xl font-bold tracking-tight">
            You're All Set!
          </h2>
          <p class="mx-auto mb-12 max-w-sm text-xl text-gray-500">
            Your venue has been listed and is now visible to thousands of
            potential guests.
          </p>

          <div class="flex flex-col gap-4">
            <a
              href="/"
              class="bg-navy hover:bg-primary w-full rounded-full py-4 text-lg font-bold text-white shadow-lg transition-all"
            >
              View My Listing
            </a>
            <a
              href="/"
              class="text-navy w-full rounded-full border-2 border-gray-100 py-4 text-lg font-bold transition-all hover:border-gray-200"
            >
              Go to Homepage
            </a>
          </div>
        </div>
      )}
    </div>
  );
}
