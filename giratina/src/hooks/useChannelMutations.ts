import { useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '../services/api';

/**
 * Mutation hook for updating an arcade's release channel
 */
export const useUpdateArcadeChannel = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      arcadeId,
      channelId,
    }: {
      arcadeId: number;
      channelId: number;
    }) => api.updateArcadeChannel(arcadeId, channelId),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['arcade', variables.arcadeId] });
      queryClient.invalidateQueries({ queryKey: ['arcades'] });
    },
  });
};

/**
 * Mutation hook for publishing a version to one or more channels
 */
export const usePublishVersion = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      gameId,
      versionId,
      channelIds,
    }: {
      gameId: number;
      versionId: number;
      channelIds: number[];
    }) => api.publishVersionToChannels(gameId, versionId, channelIds),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['game-versions', variables.gameId] });
      queryClient.invalidateQueries({ queryKey: ['game-versions'] });
    },
  });
};

/**
 * Mutation hook for unpublishing a version from all channels
 */
export const useUnpublishVersion = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ gameId, versionId }: { gameId: number; versionId: number }) =>
      api.unpublishVersion(gameId, versionId),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['game-versions', variables.gameId] });
      queryClient.invalidateQueries({ queryKey: ['game-versions'] });
    },
  });
};

/**
 * Mutation hook for creating a new release channel
 */
export const useCreateChannel = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: { name: string; description?: string }) =>
      api.createChannel(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['channels'] });
    },
  });
};

/**
 * Mutation hook for updating an existing release channel (only description can be changed)
 */
export const useUpdateChannel = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: { description?: string } }) =>
      api.updateChannel(id, data),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['channel', variables.id] });
      queryClient.invalidateQueries({ queryKey: ['channels'] });
    },
  });
};

/**
 * Mutation hook for deleting a release channel
 */
export const useDeleteChannel = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: number) => api.deleteChannel(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['channels'] });
    },
  });
};
