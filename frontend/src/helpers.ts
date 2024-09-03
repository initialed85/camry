export const parseDate = (date: string): Date => {
  return new Date(date);
};

export const formatDate = (date: Date): string => {
  const pad = (s: string): string => {
    if (s.length < 2) {
      return `0${s}`;
    }

    return s;
  };

  if (typeof date !== typeof new Date()) {
    throw new Error(
      `wanted ${new Date()} (${typeof new Date()}), got ${date} (${typeof date})`,
    );
  }

  const year = date.getFullYear().toString();
  const month = pad((date.getMonth() + 1).toString());
  const day = pad(date.getDate().toString());

  return `${year}-${month}-${day}`;
};

export const truncateDate = (date: Date): Date => {
  return parseDate(formatDate(date));
};
