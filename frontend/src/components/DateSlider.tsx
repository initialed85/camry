import Slider, { sliderClasses } from "@mui/joy/Slider";
import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { dateSliderStepMillis, dateSliderWidthMillis } from "../config";
import { formatDateWithoutMillis } from "../helpers";

export interface DateSliderProps {
  responsive: boolean;
  date: string | undefined;
  setDate: Dispatch<SetStateAction<string | undefined>>;
}

const getDateMillis = (): [number, number] => {
  const now = new Date();
  const maxDateMillisLive = now.getTime();
  const minDateMillisLive = now.getTime() - dateSliderWidthMillis;
  return [maxDateMillisLive, minDateMillisLive];
};

export default function DateSlider(props: DateSliderProps) {
  const [maxDateMillisLive, minDateMillisLive] = getDateMillis();

  const [maxDateMillis, setMaxDateMillis] = useState(maxDateMillisLive);
  const [minDateMillis, setMinDateMillis] = useState(minDateMillisLive);
  const [currentMillis, setCurrentMillis] = useState(
    maxDateMillis + dateSliderStepMillis * 1,
  );

  const [slidByUser, setSlidByUser] = useState(false);

  useEffect(() => {
    const intervalId = setInterval(() => {
      const [maxDateMillisLive, minDateMillisLive] = getDateMillis();

      setMaxDateMillis(maxDateMillisLive);
      setMinDateMillis(minDateMillisLive);

      if (!slidByUser && maxDateMillisLive - currentMillis > 1000) {
        setCurrentMillis(maxDateMillisLive);
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
  ]);

  return (
    <Slider
      variant="soft"
      size={"lg"}
      color={slidByUser ? "danger" : "success"}
      sx={{
        width: "88%",
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
  );
}
