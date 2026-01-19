import { useQuery } from '@tanstack/react-query';
import { api } from '../services/api';

/**
 * Query hook for fetching release channels
 */
export const useChannels = () => {
  return useQuery({
    queryKey: ['channels'],
    queryFn: () => api.getChannels(),
  });
};
