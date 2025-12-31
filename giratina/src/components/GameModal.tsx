import { useEffect, useState } from 'react';
import { Modal, Form, Input, Upload, message } from 'antd';
import { InboxOutlined } from '@ant-design/icons';
import type { UploadFile, UploadProps } from 'antd';
import type { Game } from '../types';
import { api } from '../services/api';

interface GameModalProps {
  open: boolean;
  mode: 'create' | 'edit';
  game?: Game;
  onSubmit: (values: any) => void;
  onCancel: () => void;
  loading?: boolean;
}

export const GameModal = ({
  open,
  mode,
  game,
  onSubmit,
  onCancel,
  loading,
}: GameModalProps) => {
  const [form] = Form.useForm();
  const [backgroundFile, setBackgroundFile] = useState<File | null>(null);
  const [fileList, setFileList] = useState<UploadFile[]>([]);
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);
  const [uploading, setUploading] = useState(false);

  useEffect(() => {
    if (open) {
      if (mode === 'edit' && game) {
        form.setFieldsValue({ name: game.name });
      } else {
        form.resetFields();
      }
      resetUpload();
    }
  }, [open, mode, game, form]);

  const resetUpload = () => {
    setBackgroundFile(null);
    setFileList([]);
    if (previewUrl) {
      URL.revokeObjectURL(previewUrl);
      setPreviewUrl(null);
    }
  };

  const handleOk = async () => {
    try {
      const values = await form.validateFields();

      if (mode === 'create') {
        if (!backgroundFile) {
          message.error('Background image is required');
          return;
        }
        values.backgroundFile = backgroundFile;
      } else if (mode === 'edit' && backgroundFile && game?.id) {
        setUploading(true);
        try {
          await api.uploadGameBackground(game.id, backgroundFile);
        } catch (error: any) {
          message.error(error.message || 'Failed to upload background');
          setUploading(false);
          return;
        }
        setUploading(false);
      }

      onSubmit(values);
    } catch (error) {
      // Validation failed
    }
  };

  const uploadProps: UploadProps = {
    onRemove: () => {
      resetUpload();
    },
    beforeUpload: (file) => {
      const isJpg = file.type === 'image/jpeg' || file.type === 'image/jpg';
      if (!isJpg) {
        message.error('Only JPG/JPEG files are allowed');
        return Upload.LIST_IGNORE;
      }

      const isLt5M = file.size / 1024 / 1024 < 5;
      if (!isLt5M) {
        message.error('Image must be smaller than 5MB');
        return Upload.LIST_IGNORE;
      }

      setBackgroundFile(file);
      setFileList([{
        uid: file.name,
        name: file.name,
        status: 'done',
        originFileObj: file,
      }]);

      // Create preview
      const reader = new FileReader();
      reader.onload = (e) => {
        if (previewUrl) {
          URL.revokeObjectURL(previewUrl);
        }
        setPreviewUrl(e.target?.result as string);
      };
      reader.readAsDataURL(file);

      return false;
    },
    fileList,
    accept: 'image/jpeg,image/jpg',
    maxCount: 1,
  };

  return (
    <Modal
      title={mode === 'create' ? 'Create Game' : 'Edit Game'}
      open={open}
      onOk={handleOk}
      onCancel={onCancel}
      confirmLoading={loading}
      destroyOnHidden
      width={600}
    >
      <Form form={form} layout="vertical">
        <Form.Item
          name="name"
          label="Game Name"
          rules={[
            { required: true, message: 'Please enter game name' },
            { min: 2, message: 'Name must be at least 2 characters' },
            { max: 255, message: 'Name must not exceed 255 characters' },
          ]}
        >
          <Input placeholder="Enter game name" />
        </Form.Item>

        <Form.Item
          label="Background Image"
          required={mode === 'create'}
        >
          {!previewUrl ? (
            <Upload.Dragger {...uploadProps}>
              <p className="ant-upload-drag-icon">
                <InboxOutlined />
              </p>
              <p className="ant-upload-text">Click or drag JPG image to upload</p>
            </Upload.Dragger>
          ) : (
            <div style={{ textAlign: 'center' }}>
              <img
                src={previewUrl}
                alt="Preview"
                style={{
                  maxWidth: '100%',
                  maxHeight: 300,
                  borderRadius: 8,
                  border: '1px solid #424242',
                }}
              />
              <button
                type="button"
                onClick={resetUpload}
                style={{
                  marginTop: 12,
                  padding: '6px 16px',
                  backgroundColor: 'transparent',
                  color: '#999',
                  border: '1px solid #424242',
                  borderRadius: '6px',
                  cursor: 'pointer',
                  fontSize: '13px',
                }}
              >
                Change Image
              </button>
            </div>
          )}
        </Form.Item>
      </Form>
    </Modal>
  );
};
