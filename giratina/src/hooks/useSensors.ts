import { useQuery } from '@tanstack/react-query';
import { api } from '../services/api';

export const SENSORS_QUERY_KEY = ['sensors'];

export function useSensors() {
  return useQuery({
    queryKey: SENSORS_QUERY_KEY,
    queryFn: () => api.getSensors(),
  });
}
