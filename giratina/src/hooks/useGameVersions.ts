import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { App } from 'antd';
import { api } from '../services/api';
import type { CreateGameVersionRequest, UpdateGameVersionRequest } from '../types';

export const GAME_VERSIONS_QUERY_KEY = ['game-versions'];

export function useGameVersions(gameId: number | null) {
  return useQuery({
    queryKey: [...GAME_VERSIONS_QUERY_KEY, gameId],
    queryFn: () => api.getGameVersions(gameId!),
    enabled: gameId !== null,
  });
}

export function useAllGameVersions(gameIds: number[]) {
  return useQuery({
    queryKey: ['all-game-versions', gameIds],
    queryFn: async () => {
      const allVersionsPromises = gameIds.map(gameId => api.getGameVersions(gameId));
      const allVersionsArrays = await Promise.all(allVersionsPromises);
      return allVersionsArrays.flat();
    },
    enabled: gameIds.length > 0,
  });
}

export function useCreateGameVersion() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: ({ gameId, data }: { gameId: number; data: CreateGameVersionRequest }) =>
      api.createGameVersion(gameId, data),
    onSuccess: (newVersion) => {
      queryClient.invalidateQueries({ queryKey: GAME_VERSIONS_QUERY_KEY });
      message.success(`Version ${newVersion.version} created successfully`);
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to create game version';
      message.error(errorMessage);
    },
  });
}

export function useUpdateGameVersion() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: ({
      gameId,
      versionId,
      data,
    }: {
      gameId: number;
      versionId: number;
      data: UpdateGameVersionRequest;
    }) => api.updateGameVersion(gameId, versionId, data),
    onSuccess: (updatedVersion) => {
      queryClient.invalidateQueries({ queryKey: GAME_VERSIONS_QUERY_KEY });
      message.success(`Version ${updatedVersion.version} updated successfully`);
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to update game version';
      message.error(errorMessage);
    },
  });
}

export function useDeleteGameVersion() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: ({ gameId, versionId }: { gameId: number; versionId: number }) =>
      api.deleteGameVersion(gameId, versionId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: GAME_VERSIONS_QUERY_KEY });
      message.success('Game version deleted successfully');
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to delete game version';
      message.error(errorMessage);
    },
  });
}
