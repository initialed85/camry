import Box from "@mui/joy/Box";
import Slider, { sliderClasses } from "@mui/joy/Slider";
import { Dispatch, SetStateAction, useEffect, useMemo, useState } from "react";
import { useQuery } from "../api";
import {
  audioStripMinScore,
  dateSliderStepMillis,
  dateSliderWidthMillis,
} from "../config";
import { audioScoreColor, formatDateWithoutMillis } from "../helpers";

export interface DateSliderProps {
  responsive: boolean;
  cameraId: string | undefined;
  date: string | undefined;
  setDate: Dispatch<SetStateAction<string | undefined>>;
}

const bucketCount =
  Math.round(dateSliderWidthMillis / dateSliderStepMillis) + 1;

const getDateMillis = (): [number, number] => {
  const now = new Date();
  const maxDateMillisLive = now.getTime();
  const minDateMillisLive = now.getTime() - dateSliderWidthMillis;
  return [maxDateMillisLive, minDateMillisLive];
};

// The strip window is hour-aligned so its query key only moves once an hour;
// the slider's own window ticks every second and would refetch every tick.
const getStripWindowMillis = (): [number, number] => {
  const now = new Date();
  const maxDateMillisLive = Math.floor(now.getTime() / dateSliderStepMillis);
  const minDateMillisLive =
    maxDateMillisLive -
    Math.round(dateSliderWidthMillis / dateSliderStepMillis);
  return [
    minDateMillisLive * dateSliderStepMillis,
    (maxDateMillisLive + 1) * dateSliderStepMillis,
  ];
};

export default function DateSlider(props: DateSliderProps) {
  const [maxDateMillisLive, minDateMillisLive] = getDateMillis();

  const [maxDateMillis, setMaxDateMillis] = useState(maxDateMillisLive);
  const [minDateMillis, setMinDateMillis] = useState(minDateMillisLive);
  const [currentMillis, setCurrentMillis] = useState(
    maxDateMillis + dateSliderStepMillis * 1,
  );

  const [slidByUser, setSlidByUser] = useState(false);

  const [stripWindowMillis, setStripWindowMillis] = useState<[number, number]>(
    getStripWindowMillis(),
  );

  useEffect(() => {
    const intervalId = setInterval(() => {
      const [maxDateMillisLive, minDateMillisLive] = getDateMillis();

      setMaxDateMillis(maxDateMillisLive);
      setMinDateMillis(minDateMillisLive);

      if (!slidByUser && maxDateMillisLive - currentMillis > 1000) {
        setCurrentMillis(maxDateMillisLive);
      }

      const [stripMinMillis, stripMaxMillis] = getStripWindowMillis();
      if (
        stripMinMillis !== stripWindowMillis[0] ||
        stripMaxMillis !== stripWindowMillis[1]
      ) {
        setStripWindowMillis([stripMinMillis, stripMaxMillis]);
      }
    }, 1_000);

    if (slidByUser && currentMillis > maxDateMillis) {
      setSlidByUser(false);
    }

    return () => {
      clearInterval(intervalId);
    };
  }, [
    currentMillis,
    maxDateMillis,
    maxDateMillisLive,
    minDateMillis,
    props,
    slidByUser,
    stripWindowMillis,
  ]);

  const { data: audioEventsData } = useQuery("get", "/api/audio-events", {
    params: {
      query: {
        camera_id__eq: props.cameraId || undefined,
        limit: 2_000,
        score__gte: audioStripMinScore,
        seen_at__gte: new Date(stripWindowMillis[0]).toISOString(),
        seen_at__lt: new Date(stripWindowMillis[1]).toISOString(),
      },
    },
    refetchInterval: 60_000,
  });

  const buckets = useMemo(() => {
    const buckets = new Map<
      number,
      { count: number; score: number; prominence: number }
    >();

    audioEventsData?.objects?.forEach((event) => {
      if (!event?.seen_at || !event?.score) {
        return;
      }

      const offsetMillis =
        new Date(event.seen_at).getTime() - stripWindowMillis[0];
      if (offsetMillis < 0) {
        return;
      }

      const index = Math.floor(offsetMillis / dateSliderStepMillis);
      if (index < 0 || index >= bucketCount) {
        return;
      }

      const bucket = buckets.get(index) || {
        count: 0,
        score: 0,
        prominence: 0,
      };
      bucket.count += 1;
      bucket.score = Math.max(bucket.score, event.score);
      bucket.prominence = Math.max(bucket.prominence, event.prominence_db || 0);

      buckets.set(index, bucket);
    });

    return buckets;
  }, [audioEventsData, stripWindowMillis]);

  const bucketMillis = (index: number): number => {
    const millis = stripWindowMillis[0] + index * dateSliderStepMillis;

    return millis > maxDateMillis ? maxDateMillis : millis;
  };

  return (
    <Box sx={{ width: "88%", display: "flex", flexDirection: "column" }}>
      <Box
        sx={{
          display: "flex",
          alignItems: "flex-end",
          height: "28px",
          mb: "2px",
        }}
      >
        {Array.from({ length: bucketCount }, (_, index) => index).map(
          (index) => {
            const bucket = buckets.get(index);
            const startMillis = bucketMillis(index);

            return (
              <Box
                key={`audio-strip-${index}`}
                title={
                  bucket
                    ? `${formatDateWithoutMillis(new Date(startMillis))} | ${
                        bucket.count
                      } events | peak +${bucket.prominence.toFixed(1)} dB | score ${bucket.score.toFixed(
                        2,
                      )}`
                    : formatDateWithoutMillis(new Date(startMillis))
                }
                onClick={() => {
                  if (!bucket) {
                    return;
                  }

                  setSlidByUser(true);
                  props.setDate(new Date(startMillis).toISOString());
                }}
                sx={{
                  flex: 1,
                  mx: "0.5px",
                  height: bucket ? `${8 + bucket.score * 92}%` : "2px",
                  backgroundColor: bucket
                    ? audioScoreColor(bucket.score)
                    : "#00000022",
                  borderRadius: "1px",
                  cursor: bucket ? "pointer" : "default",
                }}
              />
            );
          },
        )}
      </Box>
      <Slider
        variant="soft"
        size={"lg"}
        color={slidByUser ? "danger" : "success"}
        sx={{
          width: "100%",
          py: "0px",
          my: "10px",
          // Need both of the selectors to make it works on the server-side and client-side
          [`& [style*="left:0%"], & [style*="left: 0%"]`]: {
            [`&.${sliderClasses.markLabel}`]: {
              transform: "none",
            },
            [`& .${sliderClasses.valueLabel}`]: {
              left: "calc(var(--Slider-thumbSize) / 2)",
              borderBottomLeftRadius: 0,
              "&::before": {
                left: 0,
                transform: "translateY(100%)",
                borderLeftColor: "currentColor",
              },
            },
          },
          [`& [style*="left:100%"], & [style*="left: 100%"]`]: {
            [`&.${sliderClasses.markLabel}`]: {
              transform: "translateX(-100%)",
            },
            [`& .${sliderClasses.valueLabel}`]: {
              right: "calc(var(--Slider-thumbSize) / 2)",
              borderBottomRightRadius: 0,
              "&::before": {
                left: "initial",
                right: 0,
                transform: "translateY(100%)",
                borderRightColor: "currentColor",
              },
            },
          },
        }}
        value={currentMillis}
        onChange={(_, value) => {
          setSlidByUser(true);

          if (typeof value === "number" && value !== currentMillis) {
            setCurrentMillis(value);
          }
        }}
        onChangeCommitted={(_, value) => {
          if (typeof value === "number") {
            const adjustedValue = value > maxDateMillis ? maxDateMillis : value;

            const date = new Date(adjustedValue).toISOString();
            if (date !== props.date) {
              props.setDate(date);
            }
          }
        }}
        step={dateSliderStepMillis}
        min={minDateMillis}
        max={maxDateMillis + dateSliderStepMillis * 1}
        valueLabelDisplay="auto"
        valueLabelFormat={(value: number): string => {
          if (value > maxDateMillis) {
            return "(live)";
          }

          return formatDateWithoutMillis(new Date(value));
        }}
        aria-label="Always visible"
        scale={(x: number) => {
          return x; // TODO: logarithmic
        }}
      />
    </Box>
  );
}
