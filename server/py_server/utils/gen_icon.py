from PIL import Image, ImageDraw
import hashlib
import random
import io


def generate_identicon_io(seed):
    img = generate_identicon(seed)
    return image_to_io(img)


def generate_identicon(seed, size=128, block_size=10):
    # 创建一个空白图像
    img = Image.new('RGB', (size, size), color='white')
    draw = ImageDraw.Draw(img)

    # 计算哈希值
    hash = hashlib.md5(seed.encode()).hexdigest()

    # 生成随机颜色
    def random_color(seed_value):
        random.seed(seed_value)
        return tuple(random.randint(0, 255) for _ in range(3))

    # 生成一个 5x5 的像素块
    pattern = [int(hash[i], 16) % 2 for i in range(25)]
    color1 = random_color(hash[:8])
    color2 = random_color(hash[8:16])  # 使用哈希的不同部分生成另一种颜色

    for y in range(5):
        for x in range(5):
            if pattern[y * 5 + x]:
                color = color1 if (x + y) % 2 == 0 else color2  # 交替颜色
                draw.rectangle([x * block_size, y * block_size, (x + 1) * block_size, (y + 1) * block_size], fill=color)

    # 添加对称
    for y in range(5):
        for x in range(5):
            if pattern[y * 5 + x]:
                color = color1 if (x + y) % 2 == 0 else color2
                draw.rectangle(
                    [size - (x + 1) * block_size, y * block_size, size - x * block_size, (y + 1) * block_size],
                    fill=color)
                draw.rectangle(
                    [x * block_size, size - (y + 1) * block_size, (x + 1) * block_size, size - y * block_size],
                    fill=color)
                draw.rectangle([size - (x + 1) * block_size, size - (y + 1) * block_size, size - x * block_size,
                                size - y * block_size], fill=color)

    return img


def image_to_io(img, format='PNG'):
    # 创建一个 BytesIO 对象
    img_io = io.BytesIO()
    # 将图像保存到 BytesIO 对象
    img.save(img_io, format=format)
    # 将指针移动到开头
    img_io.seek(0)
    return img_io


if __name__ == '__main__':
    # 生成 Identicon 并保存
    identicon = generate_identicon('ad132fc')
    identicon.save('identicon.png')